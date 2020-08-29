# computer

AICEVOTE's main program

Document: https://computer.aicevote.com/index.html

## Sample

``` bash
curl --request POST \
  --url https://computer.aicevote.com/api/ \
  --header 'content-type: application/json' \
  --data '{"themes":[{"theme_id":0,"choices":["賛成","反対"],"dr_class":3},{"theme_id":1,"choices":["賛成","反対"],"dr_class":3}],"votes":[{"theme_id":0,"answer":1,"created_at":1598187899442,"expired_at":0},{"theme_id":1,"answer":0,"created_at":1598101579841,"expired_at":1598188014889}]}'
```
