/**
 *  If the value is found then Ok is returned, containing the index of the matching element; if the
 *  value is not found then Err is returned, containing the index where a matching element could be
 *  inserted while maintaining sorted order
 */
pub fn linear < T:Ord > (element:T, slice:&[T])->Result < usize, usize >
{
  for i
    in 0..slice.len ()
    {
      if slice
	[i] == element
	{
	return Ok (i)}
      else if slice
	[i] > element
	{
	return Err (i)}
    }
return Err (slice.len ())}

pub fn binary < T:Ord > (element:T, slice:&[T])->Result < usize, usize >
{
return binary_search_k (element, slice, 0)}

fn binary_search_k < T:Ord > (element:T, slice:&[T],
			      offset:usize)->Result < usize, usize >
{
  if slice
    ==[]
    {
    return Err (offset)}

  let midpoint = slice.len () / 2;
  if slice
    [midpoint] == element
    {
    Ok (offset + midpoint)}
  else if element
    >slice[midpoint]
    {
    binary_search_k (element, &slice[midpoint + 1..], offset + midpoint + 1)}
  else
    {
    binary_search_k (element, &slice[0..midpoint], offset)}
}
