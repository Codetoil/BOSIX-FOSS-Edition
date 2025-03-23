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
use std::borrow::Cow;
use quick_js::console::Level;
use quick_js::JsValue;

pub fn run_js(js: Cow<'static, str>)
{
    let mut context_builder = quick_js::Context::builder();
    context_builder = context_builder.console(|level: Level, args: Vec<JsValue>| {
        match level {
            Level::Error | Level::Warn => eprintln!("{}: {:?}", level, args),
            _ => println!("{}: {:?}", level, args)
        }
    });
    let context = context_builder.build().unwrap();


    // Load Javascript
    context.eval(&js).unwrap();
    println!("Javascript has finished Evaluating.");
}