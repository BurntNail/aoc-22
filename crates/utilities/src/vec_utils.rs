pub trait VecUtils<T> {
    ///Takes all of the elements which fufill the conditions, and returns the Vec of all of them
    fn take_and_return<F: Fn(&T) -> bool>(&mut self, f: F) -> Vec<T>;
    ///Maps through each element of the vec and applies the function.
    fn map<E, F: Fn(T) -> E>(self, f: F) -> Vec<E>;
}

impl<T> VecUtils<T> for Vec<T> {
    fn take_and_return<F: Fn(&T) -> bool>(&mut self, f: F) -> Vec<T> {
        //functionally equivalent to [`drain_filter`] lol
        let mut no_taken_so_far = 0;
        let mut fufilled = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            let i = i - no_taken_so_far;
            if f(&self[i]) {
                fufilled.push(self.remove(i));
                no_taken_so_far += 1;
            }
        }

        fufilled
    }

    fn map<E, F: Fn(T) -> E>(mut self, f: F) -> Vec<E> {
        let mut v = Vec::with_capacity(self.len());

        for _ in 0..self.len() {
            v.push(f(self.remove(0)));
        }

        v
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_utils::VecUtils;
    use itertools::Itertools;

    #[test]
    fn evens() {
        let mut nums = (0..10).collect_vec();
        let evens = nums.take_and_return(|x| x % 2 == 0);
        assert_eq!(nums, vec![1, 3, 5, 7, 9]);
        assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn threes() {
        let mut nums = (0..10).collect_vec();
        let threes = nums.take_and_return(|x| x % 3 == 0);
        assert_eq!(nums, vec![1, 2, 4, 5, 7, 8]);
        assert_eq!(threes, vec![0, 3, 6, 9]);
    }
}
