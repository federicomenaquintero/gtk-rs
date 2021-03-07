// Take a look at the license at the top of the repository in the LICENSE file.

use super::{InitializingType, Signal};
use crate::translate::*;
use crate::{IsA, Object, ObjectExt, ParamSpec, Type};
use std::marker;
use std::mem;

/// Trait for a type list of prerequisite object types.
pub trait PrerequisiteList {
    /// Returns the list of types for this list.
    fn types() -> Vec<ffi::GType>;
}

impl PrerequisiteList for () {
    fn types() -> Vec<ffi::GType> {
        vec![]
    }
}

impl<T: crate::ObjectType> PrerequisiteList for (T,) {
    fn types() -> Vec<ffi::GType> {
        vec![T::static_type().to_glib()]
    }
}

// Generates all the PrerequisiteList impls for prerequisite_lists of arbitrary sizes based on a list of type
// parameters like A B C. It would generate the impl then for (A, B) and (A, B, C).
macro_rules! prerequisite_list_trait(
    ($name1:ident, $name2: ident, $($name:ident),*) => (
        prerequisite_list_trait!(__impl $name1, $name2; $($name),*);
    );
    (__impl $($name:ident),+; $name1:ident, $($name2:ident),*) => (
        prerequisite_list_trait_impl!($($name),+);
        prerequisite_list_trait!(__impl $($name),+ , $name1; $($name2),*);
    );
    (__impl $($name:ident),+; $name1:ident) => (
        prerequisite_list_trait_impl!($($name),+);
        prerequisite_list_trait_impl!($($name),+, $name1);
    );
);

// Generates the impl block for PrerequisiteList on prerequisite_lists or arbitrary sizes based on its
// arguments. Takes a list of type parameters as parameters, e.g. A B C
// and then implements the trait on (A, B, C).
macro_rules! prerequisite_list_trait_impl(
    ($($name:ident),+) => (
        impl<$($name: crate::ObjectType),+> PrerequisiteList for ( $($name),+ ) {
            fn types() -> Vec<ffi::GType> {
                let mut types = Vec::new();
                prerequisite_list_trait_inner!(types, $($name)+)
            }
        }
    );
);

// Generates the inner part of the PrerequisiteList::types() implementation, which will
// basically look as follows:
//
// let mut types = Vec::new();
//
// types.push(A::static_type().to_glib());
// types.push(B::static_type().to_glib());
// [...]
// types.push(Z::static_type().to_glib());
//
// types
macro_rules! prerequisite_list_trait_inner(
    ($types:ident, $head:ident $($id:ident)+) => ({
        $types.push($head::static_type().to_glib());
        prerequisite_list_trait_inner!($types, $($id)+)
    });
    ($types:ident, $head:ident) => ({
        $types.push($head::static_type().to_glib());
        $types
    });
);

prerequisite_list_trait!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);

#[macro_export]
#[doc(hidden)]
macro_rules! object_interface_internal {
    () => {
        fn get_type() -> $crate::Type {
            static ONCE: std::sync::Once = std::sync::Once::new();
            static mut TYPE: $crate::Type = $crate::Type::INVALID;

            ONCE.call_once(|| {
                let type_ = $crate::subclass::register_interface::<Self>();
                unsafe {
                    TYPE = type_;
                }
            });

            unsafe {
                assert!(TYPE.is_valid());
                TYPE
            }
        }
    };
}

/// The central trait for defining a `GObject` interface.
///
/// Links together the type name and the interface struct for type registration and allows to hook
/// into various steps of the type registration and initialization.
///
/// This must only be implemented on `#[repr(C)]` structs and have `gobject_ffi::GTypeInterface` as
/// the first field.
///
/// See [`register_interface`] for registering an implementation of this trait
/// with the type system.
///
/// [`register_interface`]: fn.register_interface.html
pub trait ObjectInterface: Sized + 'static {
    /// `GObject` type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;

    /// Prerequisites for this interface.
    ///
    /// Any implementer of the interface must be a subclass of the prerequisites or implement them
    /// in case of interfaces.
    type Prerequisites: PrerequisiteList;

    /// Returns the `glib::Type` ID of the interface.
    ///
    /// This will register the type with the type system on the first call and is usually generated
    /// by the [`object_interface!`] macro.
    ///
    /// [`object_interface!`]: ../../macro.object_interface.html
    fn get_type() -> Type;

    /// Additional type initialization.
    ///
    /// This is called right after the type was registered and allows
    /// interfaces to do additional type-specific initialization.
    ///
    /// Optional
    fn type_init(_type_: &mut InitializingType<Self>) {}

    /// Interface initialization.
    ///
    /// This is called after `type_init` and before the first implementor
    /// of the interface is created. Interfaces can use this to do interface-
    /// specific initialization, e.g. for installing signals on the interface,
    /// and for setting default implementations of interface functions.
    ///
    /// Optional
    fn interface_init(&mut self) {}

    /// Properties installed for this interface.
    ///
    /// All implementors of the interface must provide these properties.
    fn properties() -> &'static [ParamSpec] {
        &[]
    }

    /// Signals installed for this interface.
    fn signals() -> &'static [Signal] {
        &[]
    }
}

pub trait ObjectInterfaceExt: ObjectInterface {
    /// Get interface from an instance.
    ///
    /// This will panic if `obj` does not implement the interface.
    fn from_instance<T: IsA<Object>>(obj: &T) -> &Self {
        assert!(obj.as_ref().get_type().is_a(Self::get_type()));

        unsafe {
            let klass = (*(obj.as_ptr() as *const gobject_ffi::GTypeInstance)).g_class;
            let interface =
                gobject_ffi::g_type_interface_peek(klass as *mut _, Self::get_type().to_glib());
            assert!(!interface.is_null());
            &*(interface as *const Self)
        }
    }
}

impl<T: ObjectInterface> ObjectInterfaceExt for T {}

unsafe extern "C" fn interface_init<T: ObjectInterface>(
    klass: ffi::gpointer,
    _klass_data: ffi::gpointer,
) {
    let iface = &mut *(klass as *mut T);

    let pspecs = <T as ObjectInterface>::properties();
    for pspec in pspecs {
        gobject_ffi::g_object_interface_install_property(
            iface as *mut T as *mut _,
            pspec.to_glib_none().0,
        );
    }

    let type_ = T::get_type();
    let signals = <T as ObjectInterface>::signals();
    for signal in signals {
        signal.register(type_);
    }

    iface.interface_init();
}

/// Register a `glib::Type` ID for `T`.
///
/// This must be called only once and will panic on a second call.
///
/// The [`object_interface!`] macro will create a `get_type()` function around this, which will
/// ensure that it's only ever called once.
///
/// [`object_interface!`]: ../../macro.object_interface.html
pub fn register_interface<T: ObjectInterface>() -> Type {
    unsafe {
        use std::ffi::CString;

        let type_name = CString::new(T::NAME).unwrap();
        assert_eq!(
            gobject_ffi::g_type_from_name(type_name.as_ptr()),
            gobject_ffi::G_TYPE_INVALID
        );

        let type_ = gobject_ffi::g_type_register_static_simple(
            Type::INTERFACE.to_glib(),
            type_name.as_ptr(),
            mem::size_of::<T>() as u32,
            Some(interface_init::<T>),
            0,
            None,
            0,
        );

        let prerequisites = T::Prerequisites::types();
        for prerequisite in prerequisites {
            gobject_ffi::g_type_interface_add_prerequisite(type_, prerequisite);
        }

        let type_ = from_glib(type_);

        T::type_init(&mut InitializingType::<T>(type_, marker::PhantomData));

        type_
    }
}
