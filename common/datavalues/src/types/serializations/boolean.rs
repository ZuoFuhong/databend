// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_arrow::arrow::bitmap::Bitmap;
use common_exception::ErrorCode;
use common_exception::Result;
use common_io::prelude::FormatSettings;
use opensrv_clickhouse::types::column::ArcColumnWrapper;
use opensrv_clickhouse::types::column::ColumnFrom;
use serde_json::Value;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BooleanSerializer {}

const TRUE_STR: &str = "1";
const FALSE_STR: &str = "0";

impl TypeSerializer for BooleanSerializer {
    fn serialize_value(&self, value: &DataValue, _format: &FormatSettings) -> Result<String> {
        if let DataValue::Boolean(x) = value {
            if *x {
                Ok(TRUE_STR.to_owned())
            } else {
                Ok(FALSE_STR.to_owned())
            }
        } else {
            Err(ErrorCode::BadBytes("Incorrect boolean value"))
        }
    }

    fn serialize_column(
        &self,
        column: &ColumnRef,
        _format: &FormatSettings,
    ) -> Result<Vec<String>> {
        let array: &BooleanColumn = Series::check_get(column)?;

        let result: Vec<String> = array
            .iter()
            .map(|v| {
                if v {
                    TRUE_STR.to_owned()
                } else {
                    FALSE_STR.to_owned()
                }
            })
            .collect();
        Ok(result)
    }

    fn serialize_json(&self, column: &ColumnRef, _format: &FormatSettings) -> Result<Vec<Value>> {
        let array: &BooleanColumn = Series::check_get(column)?;
        let result: Vec<Value> = array
            .iter()
            .map(|v| serde_json::to_value(v).unwrap())
            .collect();
        Ok(result)
    }

    fn serialize_clickhouse_format(
        &self,
        column: &ColumnRef,
        _format: &FormatSettings,
    ) -> Result<opensrv_clickhouse::types::column::ArcColumnData> {
        let col: &BooleanColumn = Series::check_get(column)?;
        let values: Vec<u8> = col.iter().map(|c| c as u8).collect();
        Ok(Vec::column_from::<ArcColumnWrapper>(values))
    }

    fn serialize_json_object(
        &self,
        column: &ColumnRef,
        _valids: Option<&Bitmap>,
        format: &FormatSettings,
    ) -> Result<Vec<Value>> {
        self.serialize_json(column, format)
    }

    fn serialize_json_object_suppress_error(
        &self,
        column: &ColumnRef,
        _format: &FormatSettings,
    ) -> Result<Vec<Option<Value>>> {
        let column: &BooleanColumn = Series::check_get(column)?;
        let result: Vec<Option<Value>> = column
            .iter()
            .map(|x| match serde_json::to_value(x) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect();
        Ok(result)
    }
}
