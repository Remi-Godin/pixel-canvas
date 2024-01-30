// Copyright (c) 2024 Remi A. Godin
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! A simple canvas struct that act as a 2 dimensional array.

/// A struct containing a single vector abstracted to a 2 dimensional array.
///
/// # Arguments
///
/// `data`          The actual data, stored as a `Vec<T>`
/// `width`         The width of the canvas
/// `height`        The height of the canvas
///
/// # Note
/// The canvas is repesented by a single vector of type `<T>`. The canvas can be interpreted in any
/// direction, but it was intended to start at the top left corner, and goes from left to right, up
/// to down. 
///
/// The functions provided allow access to each elemeny by row and column, as well as access to
/// iterators over the whole canvas. If you prefer the raw data, then you can access the `data`
/// field directly.
#[derive(Clone)]
pub struct Canvas<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Canvas<T> {
    /// Creates a new canvas of the specified size. The background data is the default data that
    /// will fill the canvas at initialization.
    ///
    /// Background data type must implement the `Clone` trait.
    pub fn new(width: usize, height: usize, background_data: T) -> Canvas<T> {
        let mut canvas: Canvas<T> = Canvas {
            width,
            height,
            data: Vec::with_capacity(width * height)
        };
        for _i in 0..(width * height) {
            canvas.data.push(background_data.clone());
        }
        canvas
    }


    /// Return the pixel value at the specified row and column.
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row * self.width + col)
    }

    /// Return a mutable reference to the pixel at the specified row and column.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row * self.width + col)
    }

    /// Returns an iterator over all the pixels in the canvas, starting from top left, scanning
    /// left to right, top to bottom.
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over all the pixels in the canvas, starting from top left,
    /// scanning left ot right, top to bottom.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.data.iter_mut()
    }
}
