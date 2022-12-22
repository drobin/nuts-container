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

use testx::testx;

use crate::memory::MemoryBackend;
use crate::stream::Stream;
use crate::{container::Container, memory::MemId};

use super::{assert_next_is_none, assert_prev_is_none, next, prev};

#[testx(setup = super::setup_container)]
fn empty(mut container: Container<MemoryBackend>) {
    let id_x = {
        let mut stream = Stream::create(&mut container);

        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_one)]
fn one_0(mut container: Container<MemoryBackend>, id: MemId) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id);

        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id_x);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id_x);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_one)]
fn one_1(mut container: Container<MemoryBackend>, id: MemId) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(next!(stream), &id);
        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id);
    assert_eq!(next!(stream), &id);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_two)]
fn two_0(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id1);
        assert_eq!(prev!(stream), &id_x);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id1);
    assert_eq!(prev!(stream), &id_x);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_two)]
fn two_1(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id2);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id_x);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id2);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id_x);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_two)]
fn two_2(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_three)]
fn three_0(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        assert_eq!(next!(stream), &id3);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_eq!(prev!(stream), &id_x);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id3);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_eq!(prev!(stream), &id_x);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_three)]
fn three_1(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id2);
        assert_eq!(next!(stream), &id3);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id_x);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id3);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id_x);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_three)]
fn three_2(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id3);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id_x);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id3);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id_x);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[testx(setup = super::setup_three)]
fn three_3(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        assert_eq!(next!(stream), &id3);
        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id3);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id3);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id3);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}
