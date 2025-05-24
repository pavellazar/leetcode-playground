pub fn convert(s: String, num_rows: i32) -> String {
  if num_rows == 1 {
    return s;
  }
  
  let num_rows = num_rows as usize;
  let mut rows = vec![String::new(); num_rows];
  let mut cur_row = 0;
  let mut going_down = false;

  for c in s.chars() {
    rows[cur_row].push(c);
    if cur_row == 0 || cur_row == num_rows - 1 {
      going_down = !going_down;
    }
    if going_down {
      cur_row += 1;
    } else {
      cur_row -= 1;
    }
  }

  rows.concat()
}

#[test]
fn test_convert() {
  assert_eq!(
    convert("PAYPALISHIRING".to_string(), 3),
    "PAHNAPLSIIGYIR".to_string()
  );
}
