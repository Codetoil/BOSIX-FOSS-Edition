/**
 * This file is part of BOSIX: FOSS Edition, the Official FOSS variant of the BOSIX Runtime.
 * BOSIX is a Virtual Machine Runtime Layer between Potentially Proprietary Systems and Potentially FOSS Applications.
 * Copyright (C) 2025  Anthony Michalek (Codetoil)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
#[path = "js_container.rs"] mod js_container;
use std::borrow::Cow;

fn main() {
    fn load_test() -> Cow<'static, str> {
        let bytes = include_bytes!("test.js");
        let js = String::from_utf8_lossy(bytes);
        js
    }
    let js = load_test();
    js_container::run_js(js);
}