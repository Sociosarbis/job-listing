# Job Listing

A command-line tool for fetching jobs list with Lagou's API.

Don't utilize it abusively.


## Example

**config.json**
```json
{
  "city": "城市",
  "keyword": "关键字",
  "salaryLower": 0, // 薪资下限
  "sort": 3 // 排序方式，3为按HR回复速度
}
```

```bash
job-listing.exe config.json output.json
# or
job-listing.exe config.json output.html
```