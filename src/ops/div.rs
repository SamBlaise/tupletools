use core::ops::Div;

pub trait Division<T> {
    fn div(self, other: Self) -> Self;
}

impl<T: Div<Output = T>> Division<T> for (T,) {
    fn div(self, other: Self) -> Self {
        (self.0 / other.0,)
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T) {
    fn div(self, other: Self) -> Self {
        (self.0 / other.0, self.1 / other.1)
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T) {
    fn div(self, other: Self) -> Self {
        (self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
            self.13 / other.13,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
            self.13 / other.13,
            self.14 / other.14,
        )
    }
}

impl<T: Div<Output = T>> Division<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn div(self, other: Self) -> Self {
        (
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
            self.3 / other.3,
            self.4 / other.4,
            self.5 / other.5,
            self.6 / other.6,
            self.7 / other.7,
            self.8 / other.8,
            self.9 / other.9,
            self.10 / other.10,
            self.11 / other.11,
            self.12 / other.12,
            self.13 / other.13,
            self.14 / other.14,
            self.15 / other.15,
        )
    }
}
