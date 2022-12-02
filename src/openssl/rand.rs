// MIT License
//
// Copyright (c) 2022 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

#[cfg(not(test))]
use std::os::raw::{c_int, c_uchar};

use crate::openssl::error::OpenSSLResult;
#[cfg(not(test))]
use crate::openssl::MapResult;

#[cfg(test)]
pub const RND: [u8; 1536] = [
    0x91, 0xc0, 0xb2, 0xcf, 0xe7, 0xd1, 0x1e, 0xe3, 0x19, 0x17, 0xc4, 0x48, 0xfa, 0xd5, 0x2f, 0x30,
    0xfa, 0x4a, 0x1e, 0x9c, 0xd5, 0xa4, 0x9d, 0xbe, 0x00, 0xcc, 0x42, 0x01, 0x18, 0xb5, 0xbe, 0x0f,
    0x4c, 0x71, 0x95, 0xf5, 0x8b, 0x5a, 0xa1, 0xb8, 0x2f, 0x7f, 0xe3, 0x7c, 0xd6, 0x82, 0x31, 0xb5,
    0xd4, 0x89, 0xf8, 0xce, 0xdc, 0xa3, 0xad, 0xfe, 0x73, 0x6f, 0xec, 0xfc, 0x6a, 0x4b, 0x8b, 0x7b,
    0xff, 0x42, 0x63, 0x5d, 0x7e, 0x89, 0xa6, 0xf4, 0x9b, 0xcf, 0xec, 0x48, 0x0f, 0x08, 0x37, 0x9a,
    0x41, 0xe3, 0xfe, 0xf4, 0x4a, 0x23, 0xb3, 0xe1, 0xdb, 0xf2, 0x62, 0xe2, 0xb8, 0xd0, 0xa8, 0xa1,
    0x52, 0x7d, 0xe0, 0x08, 0xfb, 0x32, 0x76, 0xc2, 0x4d, 0x7e, 0x05, 0xb8, 0x93, 0x86, 0x28, 0x78,
    0xb2, 0xb1, 0x80, 0xcd, 0xf5, 0xad, 0x8d, 0xe4, 0xbc, 0x19, 0x28, 0x9c, 0xe3, 0x4b, 0x81, 0x62,
    0x59, 0x5d, 0xaf, 0x89, 0xaa, 0x4e, 0x08, 0x89, 0xd9, 0xe2, 0x04, 0x85, 0xe8, 0x99, 0x55, 0x44,
    0x02, 0xf3, 0xad, 0x7c, 0xfe, 0x76, 0x47, 0x8f, 0xd2, 0x19, 0xba, 0x51, 0x0e, 0xab, 0xa1, 0x74,
    0xb8, 0x16, 0x7a, 0x0a, 0xf2, 0x3a, 0xa7, 0xda, 0xc0, 0x83, 0xc3, 0x7e, 0x73, 0x68, 0xdd, 0x99,
    0xe8, 0x05, 0x86, 0x72, 0x22, 0xc3, 0x98, 0xed, 0xee, 0x97, 0x2d, 0x03, 0x55, 0x91, 0x34, 0x35,
    0x84, 0x53, 0x01, 0xc5, 0x0e, 0x7b, 0xab, 0x34, 0x90, 0x56, 0xec, 0xca, 0x96, 0x3f, 0x29, 0x98,
    0x75, 0xc2, 0x05, 0x92, 0x9d, 0x8e, 0x7d, 0x1d, 0x48, 0x94, 0x52, 0xc5, 0x87, 0x7d, 0x50, 0xc4,
    0x84, 0x1c, 0x10, 0xaf, 0xea, 0x5e, 0x85, 0x41, 0x8e, 0x3c, 0x34, 0xca, 0xdc, 0x5c, 0xca, 0x8a,
    0xed, 0x09, 0x2b, 0x5e, 0x1a, 0x8e, 0xe3, 0xc8, 0x47, 0xe4, 0x28, 0x19, 0xec, 0xb5, 0x06, 0x9c,
    0x91, 0x60, 0x73, 0xc0, 0x80, 0x43, 0x50, 0x00, 0x58, 0xbc, 0x7d, 0x61, 0x82, 0x6e, 0x82, 0x1f,
    0x47, 0x27, 0x7c, 0xb7, 0x6b, 0x9f, 0x2a, 0x3b, 0x1d, 0x1f, 0xf4, 0x39, 0x32, 0x8c, 0x18, 0x13,
    0xdf, 0x9e, 0xf8, 0xc1, 0xa5, 0xfa, 0xbb, 0x47, 0x77, 0x89, 0x21, 0x4b, 0x95, 0xa4, 0x8f, 0x90,
    0xaf, 0x2b, 0xfa, 0x41, 0x5b, 0x8f, 0x76, 0xc7, 0xbe, 0x34, 0xb3, 0x2a, 0xf4, 0xf5, 0x0b, 0xfb,
    0x57, 0xa3, 0x51, 0x87, 0xd5, 0x56, 0x11, 0x9c, 0x52, 0x92, 0xbc, 0x16, 0x73, 0xc6, 0x26, 0xef,
    0xcf, 0x16, 0x70, 0x73, 0x33, 0x65, 0xef, 0xf4, 0x32, 0x45, 0x94, 0xd0, 0xf2, 0xf2, 0x1d, 0x67,
    0x74, 0xb5, 0xf2, 0x5f, 0xe2, 0xaf, 0xff, 0xba, 0xf7, 0x07, 0xbf, 0x7f, 0x86, 0xe5, 0x7d, 0xa1,
    0x2c, 0xf8, 0xda, 0x69, 0x0c, 0x07, 0x4f, 0x57, 0xda, 0x64, 0x36, 0x67, 0xf1, 0x2c, 0x59, 0x66,
    0xe5, 0xd1, 0xdf, 0x29, 0x69, 0xc2, 0x60, 0xe9, 0xc1, 0x11, 0x11, 0x0a, 0xff, 0x9b, 0x43, 0x1b,
    0xee, 0x79, 0x9c, 0x7e, 0xd0, 0x3a, 0x33, 0x66, 0xee, 0x55, 0xaa, 0xff, 0x00, 0xd1, 0x1a, 0x66,
    0x88, 0xef, 0xcc, 0xf4, 0x11, 0xac, 0xaa, 0x66, 0x99, 0xad, 0xbb, 0x66, 0x55, 0x99, 0x11, 0x88,
    0x66, 0xcb, 0xa3, 0x1b, 0xf4, 0xaa, 0xeb, 0xbe, 0xcb, 0xff, 0x55, 0xdd, 0x77, 0x00, 0x86, 0xf3,
    0x79, 0xd6, 0x99, 0xd0, 0x79, 0x01, 0xca, 0x26, 0x6b, 0xcc, 0x85, 0x22, 0xff, 0xa2, 0xa5, 0xc9,
    0x32, 0xdd, 0xb6, 0xdd, 0xa7, 0x55, 0x45, 0xbb, 0x30, 0x77, 0x38, 0x88, 0x90, 0xff, 0x1a, 0x03,
    0xa0, 0x4e, 0x55, 0x55, 0xf0, 0x6f, 0x99, 0x33, 0xaa, 0xb0, 0x42, 0x09, 0xf0, 0xff, 0xcc, 0x88,
    0x00, 0x44, 0x00, 0x33, 0xf1, 0x51, 0x11, 0x66, 0xaa, 0x44, 0x21, 0xb7, 0xff, 0xdd, 0x77, 0x11,
    0x44, 0x63, 0xc2, 0x00, 0xae, 0x4c, 0xda, 0x98, 0xdd, 0x00, 0x99, 0x88, 0x66, 0x66, 0x77, 0x77,
    0x55, 0x7d, 0xbd, 0x7b, 0x0c, 0xdd, 0xf9, 0xdd, 0x77, 0xd4, 0x13, 0x03, 0x5b, 0x7a, 0x67, 0x53,
    0x57, 0x9f, 0x79, 0xcc, 0x1a, 0xff, 0xc1, 0x57, 0xff, 0x11, 0xee, 0x66, 0xa6, 0x9a, 0x36, 0xf2,
    0x0c, 0x30, 0x91, 0xcc, 0xbb, 0x1d, 0x55, 0x99, 0x2b, 0xaa, 0x4c, 0xf9, 0x33, 0x33, 0xa9, 0xff,
    0x22, 0x44, 0x44, 0x49, 0xdd, 0x33, 0xff, 0x33, 0xaa, 0xbb, 0xd5, 0x88, 0x77, 0x88, 0xcc, 0x66,
    0x11, 0x8d, 0x66, 0xc2, 0x00, 0x77, 0xe4, 0x55, 0x24, 0x9c, 0xee, 0x48, 0xaa, 0xff, 0x22, 0x00,
    0xe5, 0xee, 0xef, 0x87, 0xac, 0x96, 0x22, 0x44, 0x4f, 0x8a, 0x77, 0xab, 0xab, 0xef, 0x20, 0xd9,
    0x06, 0xf5, 0x87, 0x6c, 0xb5, 0x77, 0x99, 0xf1, 0x43, 0x2a, 0x61, 0x82, 0x88, 0x22, 0x36, 0xa6,
    0x4d, 0xcc, 0x31, 0xee, 0x38, 0x22, 0x55, 0x11, 0xee, 0x99, 0xdd, 0xbb, 0x3d, 0xcc, 0x49, 0x5d,
    0x72, 0xa1, 0x07, 0x84, 0xcc, 0x44, 0x88, 0x55, 0x0c, 0x5b, 0xd4, 0x11, 0x32, 0xcc, 0x66, 0x44,
    0x99, 0x88, 0xff, 0xcc, 0x88, 0xff, 0xee, 0x99, 0x66, 0xe1, 0xa0, 0x06, 0x8e, 0x55, 0xbb, 0xff,
    0x33, 0xd6, 0xed, 0x8c, 0xc7, 0x8a, 0xef, 0x08, 0x99, 0x48, 0x66, 0xcb, 0x58, 0x82, 0xd9, 0x0f,
    0x66, 0x66, 0xaa, 0x22, 0xbb, 0xaa, 0xdd, 0xff, 0x22, 0x33, 0xee, 0x66, 0x1d, 0x46, 0xff, 0x99,
    0x00, 0xee, 0x11, 0x33, 0x55, 0xbb, 0x44, 0xff, 0x55, 0xdd, 0x22, 0xaa, 0xcc, 0x55, 0x88, 0x68,
    0xee, 0x77, 0x22, 0xce, 0x79, 0xdd, 0x53, 0x11, 0x55, 0xa5, 0xd6, 0xee, 0xee, 0x99, 0x00, 0x12,
    0xbb, 0x7f, 0x86, 0xf2, 0x44, 0x33, 0xf0, 0x78, 0xd2, 0x4d, 0x96, 0xfc, 0xff, 0x58, 0x85, 0xad,
    0xbb, 0x5b, 0x89, 0xd8, 0x0c, 0xee, 0x71, 0x8d, 0xba, 0x69, 0x9e, 0xc6, 0x11, 0x4c, 0x5f, 0x88,
    0x22, 0xcc, 0x77, 0xcc, 0x77, 0x9b, 0x44, 0x3b, 0x2b, 0xff, 0xfb, 0xdd, 0xbb, 0x32, 0x66, 0x73,
    0xc0, 0xc5, 0xf5, 0x90, 0xf7, 0x43, 0x53, 0xd2, 0xf8, 0xc2, 0x00, 0xcc, 0x88, 0xa8, 0x55, 0xbd,
    0x00, 0xdd, 0x89, 0x6c, 0x6e, 0x3c, 0x99, 0xc7, 0xc5, 0x74, 0xf7, 0xb4, 0x11, 0x5e, 0xee, 0x69,
    0x04, 0x72, 0xdc, 0xde, 0xfe, 0x9a, 0xd2, 0x23, 0x2c, 0xec, 0xc9, 0x04, 0x36, 0x44, 0x4e, 0xca,
    0x49, 0xdd, 0x05, 0x64, 0xdd, 0x88, 0xc6, 0x99, 0x99, 0x99, 0x77, 0x85, 0xf7, 0x9f, 0x4a, 0x6d,
    0x7f, 0x99, 0x99, 0x84, 0xe6, 0x21, 0x15, 0x88, 0x75, 0xdd, 0x76, 0x00, 0x44, 0xbf, 0x88, 0x49,
    0xc0, 0xca, 0x6c, 0xd6, 0x87, 0x78, 0x86, 0xe4, 0x44, 0xa9, 0x55, 0x50, 0x13, 0xd5, 0xcc, 0x32,
    0x0c, 0x8d, 0x29, 0xe1, 0x00, 0x57, 0xee, 0x44, 0x5d, 0x88, 0x93, 0x99, 0x76, 0x77, 0xac, 0x22,
    0x54, 0x55, 0xca, 0xcc, 0xe7, 0xca, 0x64, 0x06, 0xdd, 0x15, 0x92, 0x8e, 0x55, 0x12, 0x4f, 0xf7,
    0x55, 0x99, 0x55, 0x44, 0x00, 0x33, 0x66, 0x66, 0x00, 0xaa, 0xbb, 0x55, 0xee, 0x45, 0xf9, 0x88,
    0x90, 0x11, 0xc1, 0x6a, 0xb3, 0x2d, 0x9b, 0xa5, 0x2e, 0x88, 0x22, 0xee, 0xdf, 0x55, 0x44, 0x9d,
    0xdd, 0xaa, 0x33, 0xdd, 0x0f, 0x76, 0xcc, 0xf4, 0xad, 0x33, 0xa9, 0x00, 0xef, 0xb2, 0x94, 0xdd,
    0x77, 0xdd, 0x20, 0x24, 0xe4, 0x24, 0xee, 0xcd, 0xd4, 0xa7, 0x7e, 0xc1, 0xc0, 0x68, 0x35, 0x19,
    0xa7, 0x8a, 0xbb, 0x65, 0xcc, 0xef, 0x05, 0x99, 0x60, 0x86, 0x5e, 0x28, 0x34, 0x23, 0xee, 0x66,
    0x66, 0x45, 0x14, 0x33, 0xcd, 0xff, 0x38, 0xee, 0xe4, 0xaa, 0x66, 0xee, 0xcc, 0xff, 0x44, 0x16,
    0xaf, 0x24, 0x3, 0x5a, 0xbb, 0x54, 0x60, 0xba, 0x31, 0x34, 0x7b, 0x51, 0x46, 0xa1, 0xd, 0x99,
    0xb8, 0x6, 0x4c, 0x57, 0x63, 0x6a, 0xe1, 0x4a, 0xc2, 0x96, 0x2b, 0xca, 0x41, 0xbb, 0xda, 0x23,
    0x44, 0x96, 0x24, 0xb0, 0xdd, 0xfe, 0xa, 0xdd, 0x1d, 0x33, 0xe9, 0x1f, 0xf1, 0xc0, 0x5e, 0xaa,
    0xa1, 0x5d, 0xd5, 0x8a, 0x3e, 0x87, 0xe, 0xdd, 0xe4, 0xb8, 0x48, 0x49, 0xdb, 0x1b, 0xe5, 0x2d,
    0xe4, 0xd8, 0x5d, 0x9b, 0xc3, 0x65, 0xda, 0xbe, 0xb8, 0x0, 0x36, 0x94, 0xf3, 0x39, 0x32, 0x5d,
    0x89, 0xf, 0xa8, 0xdc, 0x37, 0x10, 0x18, 0x92, 0x1d, 0x9f, 0xb0, 0x35, 0xaa, 0x12, 0x31, 0xa8,
    0x40, 0x67, 0xab, 0x4e, 0x1f, 0x15, 0x98, 0x6d, 0x8c, 0xe7, 0xcc, 0xef, 0x6, 0x26, 0x31, 0x0,
    0x9d, 0x63, 0xb8, 0xb3, 0x6e, 0x8b, 0xf3, 0x10, 0x43, 0x70, 0x3d, 0xf9, 0x6d, 0x12, 0xf3, 0x62,
    0xa3, 0xa, 0xf6, 0x51, 0x29, 0x3, 0xd6, 0xb1, 0xf3, 0xf3, 0xe5, 0x3f, 0xbb, 0x13, 0x8e, 0x9,
    0x66, 0xa, 0x56, 0xa9, 0x0, 0x7a, 0x17, 0x31, 0xe0, 0x2c, 0xeb, 0xfd, 0x27, 0xd5, 0xb7, 0xa7,
    0xf, 0x3c, 0xb2, 0xcc, 0xee, 0x7, 0x25, 0x23, 0x7, 0xb1, 0x62, 0x29, 0x7c, 0x27, 0xc1, 0x6f,
    0x6a, 0xd2, 0x14, 0xf9, 0x4a, 0x14, 0xa0, 0xeb, 0xeb, 0x3a, 0xb6, 0xac, 0x12, 0xbe, 0x71, 0x9e,
    0xe9, 0x12, 0xbb, 0x94, 0x84, 0x32, 0xbf, 0x98, 0xe6, 0xd8, 0xef, 0xf0, 0xb8, 0xad, 0x6e, 0x4b,
    0x4f, 0x6b, 0x1a, 0xe, 0x6b, 0xf7, 0x53, 0x25, 0x83, 0x62, 0x56, 0x78, 0xa4, 0x6, 0xac, 0xfa,
    0x74, 0xcb, 0xb9, 0x3c, 0x9d, 0x2f, 0xe4, 0xcc, 0x90, 0x80, 0xaf, 0x58, 0xa3, 0x85, 0xec, 0xa9,
    0x73, 0x23, 0xd, 0x1c, 0xd5, 0xe0, 0xd6, 0xf9, 0x4d, 0x8d, 0xad, 0x3d, 0x1a, 0xe, 0x60, 0xfc,
    0xfb, 0x6a, 0xdb, 0xb6, 0x5, 0x91, 0xb9, 0x54, 0xa4, 0x9d, 0x92, 0x8f, 0x71, 0x7b, 0x9b, 0x70,
    0xf2, 0xaa, 0x85, 0xfd, 0x91, 0x36, 0x5b, 0xa, 0x7, 0xd4, 0x9c, 0x96, 0xe7, 0xc5, 0x23, 0x74,
    0xa, 0xe6, 0xec, 0x76, 0x81, 0x1a, 0x6b, 0xba, 0xc1, 0x6, 0x83, 0xf, 0xae, 0xf7, 0xc2, 0xd4,
    0x2b, 0x62, 0xcb, 0x52, 0xd8, 0xbb, 0x6a, 0xcb, 0x24, 0xc5, 0xfb, 0xb3, 0x91, 0x1f, 0x84, 0x3d,
    0x6, 0x9f, 0x2c, 0xd2, 0x6e, 0x2c, 0x9c, 0x1a, 0x26, 0x4c, 0x30, 0x28, 0x35, 0x52, 0x2, 0x6b,
    0xc4, 0x88, 0x4f, 0x38, 0x56, 0x73, 0x14, 0xa, 0xf1, 0x5f, 0xe5, 0x38, 0x60, 0xd0, 0x52, 0x88,
    0x8, 0xca, 0x83, 0xc9, 0x8a, 0x30, 0x84, 0x66, 0x9a, 0x98, 0x39, 0x4f, 0xbb, 0x98, 0xa6, 0xd2,
    0xe2, 0xee, 0x1b, 0x5e, 0xe4, 0xc4, 0x4d, 0xc, 0x10, 0xdc, 0xfc, 0x2b, 0x91, 0x28, 0x27, 0x93,
    0xf7, 0xea, 0xc8, 0x42, 0xbd, 0xb2, 0xaf, 0xb8, 0xff, 0x6a, 0x71, 0xf0, 0x71, 0x9a, 0xeb, 0xe1,
    0xe2, 0x9d, 0xaf, 0xa0, 0x9, 0x14, 0x86, 0x8e, 0x8a, 0x53, 0xb3, 0x39, 0xff, 0x6, 0xf9, 0x94,
    0x6a, 0x7, 0xe0, 0xe3, 0x8a, 0x9c, 0x61, 0x6d, 0x60, 0xa6, 0x55, 0xd8, 0xb8, 0x65, 0xe8, 0xc3,
    0x62, 0x37, 0x14, 0x31, 0x88, 0xd1, 0x8a, 0x3c, 0x9b, 0xa0, 0xe2, 0x73, 0x91, 0x2b, 0x59, 0x77,
    0xb7, 0x3f, 0xa8, 0xb7, 0xf6, 0x1c, 0xd6, 0x28, 0x3e, 0x7c, 0xd7, 0x7b, 0xdc, 0xc8, 0xf3, 0xfc,
    0x9, 0x3f, 0xe0, 0x12, 0x18, 0xa, 0xed, 0xd1, 0x61, 0x51, 0x3b, 0xd2, 0x2, 0xa8, 0x68, 0xe6,
    0x41, 0xea, 0x51, 0x5d, 0x40, 0x4f, 0xe3, 0x19, 0x0, 0x93, 0xcc, 0x74, 0xcc, 0x48, 0x3c, 0x4b,
    0x61, 0x87, 0x45, 0x7a, 0x95, 0xfb, 0xdb, 0x19, 0x7a, 0x62, 0x46, 0x5, 0x31, 0x76, 0x3a, 0xdf,
];

extern "C" {
    #[cfg(not(test))]
    fn RAND_bytes(buf: *mut c_uchar, num: c_int) -> c_int;
}

#[cfg(test)]
pub fn rand_bytes(buf: &mut [u8]) -> OpenSSLResult<()> {
    assert!(buf.len() <= RND.len());
    buf.clone_from_slice(&RND[..buf.len()]);
    Ok(())
}

#[cfg(not(test))]
pub fn rand_bytes(buf: &mut [u8]) -> OpenSSLResult<()> {
    unsafe { RAND_bytes(buf.as_mut_ptr(), buf.len() as c_int) }.map_result(|_| ())
}

pub fn rand_u32() -> OpenSSLResult<u32> {
    let mut bytes = [0; 4];

    rand_bytes(&mut bytes)?;

    Ok(u32::from_be_bytes(bytes))
}
