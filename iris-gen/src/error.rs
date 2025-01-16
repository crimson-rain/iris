/* FILENAME: error.rs
 * 
 * DESCRIPTION 
 * This file defines error types, using thiserror library.
 * This library allows us to avoid boiler plate code, 
 * when defining a variety of errors which can occur in the code.
 * 
 * NOTES
 * 
 * AUTHOR:    Rezwan Rahman (RAH22529097)
 * CREATED:   16/11/2024
 * MODIFIED:  16/11/2024
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {

}

#[cfg(test)]
mod tests {

}