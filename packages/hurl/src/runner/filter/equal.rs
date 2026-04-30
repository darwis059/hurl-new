/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2026 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use hurl_core::ast::{SourceInfo, Template};

use crate::runner::template::eval_template;
use crate::runner::{RunnerError, RunnerErrorKind, Value, VariableSet};

/// Returns true or false by comparing the string `value` with the evaluated `other` template.
pub fn eval_equal(
    value: &Value,
    variables: &VariableSet,
    source_info: SourceInfo,
    assert: bool,
    other: &Template,
) -> Result<Option<Value>, RunnerError> {
    match value {
        Value::String(s) => {
            let other = eval_template(other, variables)?;
            let result = s == &other;
            Ok(Some(Value::Bool(result)))
        }
        v => {
            let kind = RunnerErrorKind::FilterInvalidInputType {
                actual: v.kind().to_string(),
                expected: "string".to_string(),
            };
            Err(RunnerError::new(source_info, kind, assert))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use hurl_core::ast::{Filter, FilterValue, SourceInfo, Template, TemplateElement, Whitespace};
//     use hurl_core::reader::Pos;
//     use hurl_core::types::ToSource;

//     use crate::runner::filter::eval::eval_filter;
//     use crate::runner::{Value, VariableSet};

//     #[test]
//     fn eval_filter_split() {
//         let variables = VariableSet::new();
//         let filter = Filter {
//             source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
//             value: FilterValue::Split {
//                 sep: Template::new(
//                     Some('"'),
//                     vec![TemplateElement::String {
//                         value: ",".to_string(),
//                         source: ",".to_source(),
//                     }],
//                     SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
//                 ),
//                 space0: Whitespace {
//                     value: String::new(),
//                     source_info: SourceInfo::new(Pos::new(0, 0), Pos::new(0, 0)),
//                 },
//             },
//         };

//         assert_eq!(
//             eval_filter(
//                 &filter,
//                 &Value::String("1,2,3".to_string()),
//                 &variables,
//                 false
//             )
//             .unwrap()
//             .unwrap(),
//             Value::List(vec![
//                 Value::String("1".to_string()),
//                 Value::String("2".to_string()),
//                 Value::String("3".to_string()),
//             ])
//         );
//     }
// }
