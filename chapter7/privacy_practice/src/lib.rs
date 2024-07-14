pub mod outermost { //private
    pub fn middle_function() {}
    fn middle_secret_function() {}  //not public
    mod inside {
    pub fn inner_function() {}
    fn secret_function() {}
    }
    }

fn try_me() {
    outermost::middle_function();
    ::outermost::middle_secret_function()
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

//try_me is allowed access to outermostbecause it is in the current root module
// middle function witll also work because it is public
//However outermost::middle_secret_function will cause a complication error because it is private


//inside module is private with no child modules so it can be access only by its current module  outermost
// try_me function is not allowed to call 'outermost::inside::inner_function' nor 'outermost::inside::secret::function'

// What if the inside module were public?
// What if outermost were public and inside were private?
// What if, in the body of inner_function, you called
// ::outermost::middle_secret_function()? (The two colons at the beginning
// mean that we want to refer to the modules starting from the root
// module.)