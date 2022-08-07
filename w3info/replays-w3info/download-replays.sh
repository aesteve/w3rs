#!/bin/bash
for replay_id in {72400..72656}
do
  echo "Downloading replay #$replay_id"
  curl -s -XGET https://warcraft3.info/api/v1/replays/$replay_id -H 'Accept:application/json' >> ./analysed.jsonl
  echo "\n\t" >> ./analysed.jsonl
  curl -s -XGET https://warcraft3.info/api/v1/replays/$replay_id/parse -H 'Accept:application/json' >> ./analysed.jsonl
  echo "\n" >> ./analysed.jsonl
  curl -s -XGET https://warcraft3.info/api/v1/replays/72665/download > ./analysed/$replay_id.w3g
done