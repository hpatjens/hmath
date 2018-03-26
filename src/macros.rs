
macro_rules! implement_binary_assign_operator {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) {
            $body:expr
        }
    ) => {
        // val
        impl<T> $operator_trait<$type_rhs> for $type where T: Base {
            fn $function(&mut self, other: $type_rhs) {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        // &val
        impl<'a, T> $operator_trait<&'a $type_rhs> for $type where T: Base {
            fn $function(&mut self, other: &'a $type_rhs) {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // &mut val
        impl<'a, T> $operator_trait<&'a mut $type_rhs> for $type where T: Base {
            fn $function(&mut self, other: &'a mut $type_rhs) {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }
    }
}

macro_rules! implement_unary_operator {
    // With simple constraint
    ($operator_trait:ident for $type:ty where T: $contraint_trait:ident,
        fn $function:ident($self:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        impl<T> $operator_trait for $type where T: Base + $contraint_trait {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a $type where T: Base + $contraint_trait {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a mut $type where T: Base + $contraint_trait {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }
    };
    // With constraint that defines "Output"
    ($operator_trait:ident for $type:ty where T: $contraint_trait:ident<Output=T>,
        fn $function:ident($self:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        impl<T> $operator_trait for $type where T: Base + $contraint_trait<Output=T> {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a $type where T: Base + $contraint_trait<Output=T> {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }

        impl<'a, T> $operator_trait for &'a mut $type where T: Base + $contraint_trait<Output=T> {
            type Output = $type;
            #[inline]
            fn $function(self) -> $type { let $self = self; $body }
        }
    }
}
 
macro_rules! implement_binary_operator {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        // self op other
        impl<T> $operator_trait<$type_rhs> for $type where T: Base {
            type Output = $result_type;
            #[inline] 
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        // self op &other
        impl<'b, T> $operator_trait<&'b $type_rhs> for $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // self op &mut other
        impl<'b, T> $operator_trait<&'b mut $type_rhs> for $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // &self op other
        impl<'a, T> $operator_trait<$type_rhs> for &'a $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &self op &other
        impl<'a, T> $operator_trait<&'a $type_rhs> for &'a $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'a $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &self op &mut other
        impl<'a, T> $operator_trait<&'a mut $type_rhs> for &'a $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'a mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op other
        impl<'a, T> $operator_trait<$type_rhs> for &'a mut $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &mut self op &other
        impl<'a, 'b, T> $operator_trait<&'b $type_rhs> for &'a mut $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op &mut other
        impl<'a, 'b, T> $operator_trait<&'b mut $type_rhs> for &'a mut $type where T: Base {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }
    }
}

macro_rules! implement_binary_operator_non_generic {
    ($operator_trait:ident<$type_rhs:ty> for $type:ty,
        fn $function:ident($lhs:ident, $rhs:ident) -> $result_type:ty {
            $body:expr
        }
    ) => {
        // self op other
        impl $operator_trait<$type_rhs> for $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, other); $body
            }
        }

        // self op &other
        impl<'b> $operator_trait<&'b $type_rhs> for $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // self op &mut other
        impl<'b> $operator_trait<&'b mut $type_rhs> for $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (self, *other); $body
            }
        }

        // &self op other
        impl<'a> $operator_trait<$type_rhs> for &'a $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &self op &other
        impl<'a, 'b> $operator_trait<&'b $type_rhs> for &'a $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &self op &mut other
        impl<'a, 'b> $operator_trait<&'b mut $type_rhs> for &'a $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op other
        impl<'a> $operator_trait<$type_rhs> for &'a mut $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, other); $body
            }
        }

        // &mut self op &other
        impl<'a, 'b> $operator_trait<&'b $type_rhs> for &'a mut $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }

        // &mut self op &mut other
        impl<'a, 'b> $operator_trait<&'b mut $type_rhs> for &'a mut $type {
            type Output = $result_type;
            #[inline]
            fn $function(self, other: &'b mut $type_rhs) -> $result_type {
                let ($lhs, $rhs) = (*self, *other); $body
            }
        }
    }
}

macro_rules! implement_specific_operators_for_vector {
    ($type:ident { $($member:ident),+ } for $specific_type:ty) => {
        // s * v
        implement_binary_operator_non_generic!(Mul<$type<$specific_type>> for $specific_type,
            fn mul(scalar, vector) -> $type<$specific_type> {
                $type::new( $(scalar * vector.$member),* )
            }
        );

        // s / v
        implement_binary_operator_non_generic!(Div<$type<$specific_type>> for $specific_type,
            fn div(scalar, vector) -> $type<$specific_type> {
                $type::new( $(scalar / vector.$member),* )
            }
        );
    }
}
