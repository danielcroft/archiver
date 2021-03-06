#![deny(unused_must_use)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate chrono;
extern crate digest;
extern crate hashing_copy;
extern crate hex;
extern crate libusb;
extern crate ptp;
extern crate regex;
extern crate reqwest;
extern crate sendgrid;
extern crate serde_json;
extern crate sha2;
extern crate toml;
extern crate walkdir;

/// Currently, we have vendored Dropbox's implementation of their content hashing algorithm.
/// There's an outstanding pull request open to upstream the changes I made, at which point this
/// can go.
pub mod dropbox_content_hasher;

/// Details pertaining to parsing the configuration file, as well as constructing the internal
/// objects specified by the configuration.
pub mod config;

/// The global context object that is threaded throughout the run of the program. This module also
/// deals with some implementation details, like ensuring that the staging directory exists as part
/// of standing up the context.
pub mod ctx;

/// Some helpers to abstract over the various types of devices that we can interact with. Much of
/// this will probably go away at some point, and Device will become a trait instead of the enum
/// that it is today.
///
/// This module also contains the logic for simply enumerating all currently attached devices as
/// part of generating a plan for an upload run.
pub mod device;

/// Our interface to the dropbox API. This should really be it's own crate, but until I have the
/// enthusiasm to implement more than the bare minimum archiver needs, it will remain vendored
/// here.
pub mod dropbox;

/// Flysight specific code. This mostly relates to parsing out the filenames that flysights create.
mod flysight;

/// Contains the MailReport trait which all mailers must implement, as well as the archiver
/// specific glue for the `SGClient` object we use from the `sendgrid` crate.
pub mod mailer;

/// Code relating to the `mass_storage` device type. This is any device that can be mounted to the
/// local filesystem.
mod mass_storage;

/// Contains the `MountablePeripheral` trait, common to `flysight`s and `mass_storage`s. This is
/// simply the glue that makes it easy to check if they're currently present.
mod peripheral;

/// Our bindings to the ptp crate, which we use to talk to devices like Gopros over USB, allowing
/// us to avoid having to pull the SD card in order to upload footage.
pub mod ptp_device;

/// Bindings written for the pushover notification service. Like Dropbox, this can and should be
/// extracted into its own crate, but until it's a little more stable it can remain here.
pub mod pushover;

/// Contains the `Notify` trait, which all notifiers must implement. Contains impls, as well as a
/// little local glue to bind `config` and `pushover` together.
pub mod pushover_notifier;

/// Contains the machinery for generating an upload report. This handles both building the report
/// object up in memory, as well as rendering it to something we can mail to a user.
mod reporting;

/// Machinry for locally staging files from attached devices. It includes the `Staging` trait,
/// which when implemented allows for not implementing some of the heavy lifting.
mod staging;

/// Contains the logic for consuming the locally staged files and uploading them to the selected
/// storage backend. Also deals with deduping (Locally hashing files to ensure that we're not
/// pointlessly uploading things that are already there) and cleaning up the local staging area.
pub mod storage;

mod version;
/// What version of archiver do you have :)
pub use version::VERSION;

#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "web")]
#[macro_use]
extern crate diesel;
#[cfg(feature = "web")]
extern crate bcrypt;
#[cfg(feature = "web")]
extern crate rand;
#[cfg(feature = "web")]
extern crate rocket;
