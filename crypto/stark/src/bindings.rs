/* automatically generated by rust-bindgen 0.57.0 */

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[repr(C)]
    pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
    impl<T> __BindgenUnionField<T> {
        #[inline]
        pub const fn new() -> Self {
            __BindgenUnionField(::std::marker::PhantomData)
        }

        #[inline]
        pub unsafe fn as_ref(&self) -> &T {
            ::std::mem::transmute(self)
        }

        #[inline]
        pub unsafe fn as_mut(&mut self) -> &mut T {
            ::std::mem::transmute(self)
        }
    }
    impl<T> ::std::default::Default for __BindgenUnionField<T> {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }
    impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self::new()
        }
    }
    impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
    impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            fmt.write_str("__BindgenUnionField")
        }
    }
    impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
        fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
    }
    impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
        fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
            true
        }
    }
    impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
    #[allow(unused_imports)]
    use self::super::root;
    pub mod std {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub type size_t = ::std::os::raw::c_ulong;
        #[repr(C)]
        pub struct basic_string<_CharT> {
            pub _M_dataplus:      root::std::basic_string__Alloc_hider,
            pub _M_string_length: root::std::basic_string_size_type,
            pub __bindgen_anon_1: root::std::basic_string__bindgen_ty_2<_CharT>,
            pub _phantom_0:       ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type basic_string__Char_alloc_type = [u8; 0usize];
        pub type basic_string__Alloc_traits = root::__gnu_cxx::__alloc_traits;
        pub type basic_string_traits_type<_Traits> = _Traits;
        pub type basic_string_value_type = [u8; 0usize];
        pub type basic_string_allocator_type = root::std::basic_string__Char_alloc_type;
        pub type basic_string_size_type = [u8; 0usize];
        pub type basic_string_difference_type = [u8; 0usize];
        pub type basic_string_reference = [u8; 0usize];
        pub type basic_string_const_reference = [u8; 0usize];
        pub type basic_string_pointer = [u8; 0usize];
        pub type basic_string_const_pointer = [u8; 0usize];
        pub type basic_string_iterator =
            root::__gnu_cxx::__normal_iterator<root::std::basic_string_pointer>;
        pub type basic_string_const_iterator =
            root::__gnu_cxx::__normal_iterator<root::std::basic_string_const_pointer>;
        pub type basic_string_const_reverse_iterator =
            root::std::reverse_iterator<root::std::basic_string_const_iterator>;
        pub type basic_string_reverse_iterator =
            root::std::reverse_iterator<root::std::basic_string_iterator>;
        pub type basic_string___const_iterator = root::std::basic_string_const_iterator;
        #[repr(C)]
        pub struct basic_string__Alloc_hider {
            pub _M_p: root::std::basic_string_pointer,
        }
        pub const basic_string__S_local_capacity: i32 = 0;
        pub type basic_string__bindgen_ty_1 = i32;
        #[repr(C)]
        pub struct basic_string__bindgen_ty_2<_CharT> {
            pub _M_local_buf:          root::__BindgenUnionField<*mut _CharT>,
            pub _M_allocated_capacity: root::__BindgenUnionField<root::std::basic_string_size_type>,
            pub bindgen_union_field:   u64,
            pub _phantom_0:            ::std::marker::PhantomData<::std::cell::UnsafeCell<_CharT>>,
        }
        pub type string = root::std::basic_string<::std::os::raw::c_char>;
        pub type integral_constant_value_type<_Tp> = _Tp;
        pub type integral_constant_type = u8;
        pub type true_type = u8;
        pub type false_type = u8;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __and_ {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct is_empty {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct make_unsigned {
            pub _address: u8,
        }
        pub type make_unsigned_type = u8;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __detector {
            pub _address: u8,
        }
        pub type __detector_value_t = root::std::false_type;
        pub type __detector_type<_Default> = _Default;
        pub type __detected_or = root::std::__detector;
        pub type __detected_or_t = root::std::__detected_or;
        #[repr(C)]
        #[derive(Debug)]
        pub struct iterator {
            pub _address: u8,
        }
        pub type iterator_iterator_category<_Category> = _Category;
        pub type iterator_value_type<_Tp> = _Tp;
        pub type iterator_difference_type<_Distance> = _Distance;
        pub type iterator_pointer<_Pointer> = _Pointer;
        pub type iterator_reference<_Reference> = _Reference;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __iterator_traits {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct iterator_traits {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct __replace_first_arg {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct __ptr_traits_elem_1 {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct __ptr_traits_elem {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct pointer_traits {
            pub _address: u8,
        }
        pub type pointer_traits___difference_type = [u8; 0usize];
        #[repr(C)]
        #[derive(Debug)]
        pub struct pointer_traits___rebind {
            pub _address: u8,
        }
        pub type pointer_traits_pointer<_Ptr> = _Ptr;
        pub type pointer_traits_difference_type = root::std::__detected_or_t;
        pub type pointer_traits_rebind = root::std::pointer_traits___rebind;
        #[repr(C)]
        #[derive(Debug)]
        pub struct reverse_iterator<_Iterator> {
            pub current:    _Iterator,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
        }
        pub type reverse_iterator___traits_type = root::std::iterator_traits;
        pub type reverse_iterator_iterator_type<_Iterator> = _Iterator;
        pub type reverse_iterator_pointer = root::std::reverse_iterator___traits_type;
        pub type reverse_iterator_difference_type = root::std::reverse_iterator___traits_type;
        pub type reverse_iterator_reference = root::std::reverse_iterator___traits_type;
        pub type streamoff = ::std::os::raw::c_long;
        #[repr(C)]
        #[derive(Debug)]
        pub struct fpos<_StateT> {
            pub _M_off:     root::std::streamoff,
            pub _M_state:   _StateT,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_StateT>>,
        }
        pub type streampos = root::std::fpos<root::__mbstate_t>;
        #[repr(C)]
        #[derive(Debug)]
        pub struct char_traits {
            pub _address: u8,
        }
        pub type __allocator_base = root::__gnu_cxx::new_allocator;
        #[repr(C)]
        #[derive(Debug)]
        pub struct allocator {
            pub _address: u8,
        }
        pub type allocator_value_type<_Tp> = _Tp;
        pub type allocator_size_type = root::std::size_t;
        pub type allocator_difference_type = isize;
        pub type allocator_pointer<_Tp> = *mut _Tp;
        pub type allocator_const_pointer<_Tp> = *const _Tp;
        pub type allocator_reference<_Tp> = *mut _Tp;
        pub type allocator_const_reference<_Tp> = *const _Tp;
        #[repr(C)]
        #[derive(Debug)]
        pub struct allocator_rebind {
            pub _address: u8,
        }
        pub type allocator_rebind_other = root::std::allocator;
        pub type allocator_propagate_on_container_move_assignment = root::std::true_type;
        pub type allocator_is_always_equal = root::std::true_type;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __allocator_traits_base {
            pub _address: u8,
        }
        #[repr(C)]
        #[derive(Debug)]
        pub struct __allocator_traits_base___rebind {
            pub _address: u8,
        }
        pub type __allocator_traits_base___pointer = [u8; 0usize];
        pub type __allocator_traits_base___c_pointer = [u8; 0usize];
        pub type __allocator_traits_base___v_pointer = [u8; 0usize];
        pub type __allocator_traits_base___cv_pointer = [u8; 0usize];
        pub type __allocator_traits_base___pocca = [u8; 0usize];
        pub type __allocator_traits_base___pocma = [u8; 0usize];
        pub type __allocator_traits_base___pocs = [u8; 0usize];
        pub type __allocator_traits_base___equal = [u8; 0usize];
        pub type __alloc_rebind = root::std::__allocator_traits_base;
        #[repr(C)]
        #[derive(Debug)]
        pub struct allocator_traits {
            pub _address: u8,
        }
        pub type allocator_traits_allocator_type<_Alloc> = _Alloc;
        pub type allocator_traits_value_type = [u8; 0usize];
        pub type allocator_traits_pointer = root::std::__detected_or_t;
        #[repr(C)]
        #[derive(Debug)]
        pub struct allocator_traits__Ptr {
            pub _address: u8,
        }
        pub type allocator_traits__Ptr_type = [u8; 0usize];
        #[repr(C)]
        #[derive(Debug)]
        pub struct allocator_traits__Diff {
            pub _address: u8,
        }
        pub type allocator_traits__Diff_type = root::std::pointer_traits;
        #[repr(C)]
        #[derive(Debug)]
        pub struct allocator_traits__Size {
            pub _address: u8,
        }
        pub type allocator_traits_const_pointer = [u8; 0usize];
        pub type allocator_traits_void_pointer = root::std::allocator_traits__Ptr;
        pub type allocator_traits_const_void_pointer = root::std::allocator_traits__Ptr;
        pub type allocator_traits_difference_type = [u8; 0usize];
        pub type allocator_traits_size_type = [u8; 0usize];
        pub type allocator_traits_propagate_on_container_copy_assignment =
            root::std::__detected_or_t;
        pub type allocator_traits_propagate_on_container_move_assignment =
            root::std::__detected_or_t;
        pub type allocator_traits_propagate_on_container_swap = root::std::__detected_or_t;
        pub type allocator_traits_is_always_equal = root::std::__detected_or_t;
        pub type allocator_traits_rebind_alloc = root::std::__alloc_rebind;
        pub type allocator_traits_rebind_traits = root::std::allocator_traits;
        #[repr(C)]
        #[derive(Debug)]
        pub struct allocator_traits___construct_helper {
            pub _address: u8,
        }
        pub type allocator_traits___construct_helper_type<_Alloc> = _Alloc;
        pub type allocator_traits___has_construct = root::std::allocator_traits___construct_helper;
    }
    pub mod __gnu_cxx {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __normal_iterator<_Iterator> {
            pub _M_current: _Iterator,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
        }
        pub type __normal_iterator___traits_type = root::std::iterator_traits;
        pub type __normal_iterator_iterator_type<_Iterator> = _Iterator;
        pub type __normal_iterator_iterator_category =
            root::__gnu_cxx::__normal_iterator___traits_type;
        pub type __normal_iterator_value_type = root::__gnu_cxx::__normal_iterator___traits_type;
        pub type __normal_iterator_difference_type =
            root::__gnu_cxx::__normal_iterator___traits_type;
        pub type __normal_iterator_reference = root::__gnu_cxx::__normal_iterator___traits_type;
        pub type __normal_iterator_pointer = root::__gnu_cxx::__normal_iterator___traits_type;
        #[repr(C)]
        #[derive(Debug)]
        pub struct _Char_types {
            pub _address: u8,
        }
        pub type _Char_types_int_type = ::std::os::raw::c_ulong;
        pub type _Char_types_pos_type = root::std::streampos;
        pub type _Char_types_off_type = root::std::streamoff;
        pub type _Char_types_state_type = root::__mbstate_t;
        #[repr(C)]
        #[derive(Debug)]
        pub struct char_traits {
            pub _address: u8,
        }
        pub type char_traits_char_type<_CharT> = _CharT;
        pub type char_traits_int_type = root::__gnu_cxx::_Char_types;
        pub type char_traits_pos_type = root::__gnu_cxx::_Char_types;
        pub type char_traits_off_type = root::__gnu_cxx::_Char_types;
        pub type char_traits_state_type = root::__gnu_cxx::_Char_types;
        #[repr(C)]
        #[derive(Debug)]
        pub struct new_allocator {
            pub _address: u8,
        }
        pub type new_allocator_value_type<_Tp> = _Tp;
        pub type new_allocator_size_type = root::std::size_t;
        pub type new_allocator_difference_type = isize;
        pub type new_allocator_pointer<_Tp> = *mut _Tp;
        pub type new_allocator_const_pointer<_Tp> = *const _Tp;
        pub type new_allocator_reference<_Tp> = *mut _Tp;
        pub type new_allocator_const_reference<_Tp> = *const _Tp;
        #[repr(C)]
        #[derive(Debug)]
        pub struct new_allocator_rebind {
            pub _address: u8,
        }
        pub type new_allocator_rebind_other = root::__gnu_cxx::new_allocator;
        pub type new_allocator_propagate_on_container_move_assignment = root::std::true_type;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __alloc_traits {
            pub _address: u8,
        }
        pub type __alloc_traits_allocator_type<_Alloc> = _Alloc;
        pub type __alloc_traits__Base_type = root::std::allocator_traits;
        pub type __alloc_traits_value_type = root::__gnu_cxx::__alloc_traits__Base_type;
        pub type __alloc_traits_pointer = root::__gnu_cxx::__alloc_traits__Base_type;
        pub type __alloc_traits_const_pointer = root::__gnu_cxx::__alloc_traits__Base_type;
        pub type __alloc_traits_size_type = root::__gnu_cxx::__alloc_traits__Base_type;
        pub type __alloc_traits_difference_type = root::__gnu_cxx::__alloc_traits__Base_type;
        pub type __alloc_traits_reference = *mut root::__gnu_cxx::__alloc_traits_value_type;
        pub type __alloc_traits_const_reference = *const root::__gnu_cxx::__alloc_traits_value_type;
        pub type __alloc_traits___is_custom_pointer = root::std::__and_;
        #[repr(C)]
        #[derive(Debug)]
        pub struct __alloc_traits_rebind {
            pub _address: u8,
        }
        pub type __alloc_traits_rebind_other = root::__gnu_cxx::__alloc_traits__Base_type;
    }
    pub type size_t = ::std::os::raw::c_ulong;
    #[repr(C)]
    pub struct __mbstate_t {
        pub __count: ::std::os::raw::c_int,
        pub __value: root::__mbstate_t__bindgen_ty_1,
    }
    #[repr(C)]
    pub struct __mbstate_t__bindgen_ty_1 {
        pub __wch:               root::__BindgenUnionField<::std::os::raw::c_uint>,
        pub __wchb:              root::__BindgenUnionField<[::std::os::raw::c_char; 4usize]>,
        pub bindgen_union_field: u32,
    }
    pub type __uint16_t = ::std::os::raw::c_ushort;
    extern "C" {
        pub fn execute(
            assemblyFile: root::std::string,
            primaryTapeFile: root::std::string,
            auxTapeFile: root::std::string,
            t: root::size_t,
            prover: bool,
            address: *const root::std::string,
            port_number: u16,
            verbose: bool,
            session: *const root::std::string,
            macros_file: *const root::std::string,
        );
    }
}
