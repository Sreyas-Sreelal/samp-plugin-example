//#[macro_use] extern crate samp_sdk;
//extern crate memcache;
mod plugin;

use crate::plugin::Memcached;
use samp_sdk::new_plugin;

new_plugin!(Memcached);
