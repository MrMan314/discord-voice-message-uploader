set -x
file=$1
chan=$2
auth=$3
fs=`wc -c < $file`
resp=`curl "https://canary.discord.com/api/v9/channels/$chan/attachments" \
	-H 'accept: */*' \
	-H "authorization: $auth" \
	-H 'content-type: application/json' \
	--data-raw '{"files":[{"filename":"voice-message.ogg","file_size":'$fs'}]}' \
	--compressed \
	--silent`
url=`jq -r ".attachments[0].upload_url" <<< $resp`
fn=`jq -r ".attachments[0].upload_filename" <<< $resp`
curl "$url" \
	-X 'PUT' \
	--data-binary "@$file" \
	--compressed
curl -i -s -k -X $'POST' \
	-H $'Host: discord.com' \
	-H $'Content-Type: application/json' \
	-H $'Accept: */*' \
	-H $'Authorization: '$auth \
	-H $'Accept-Encoding: gzip, deflate' \
	-H $'X-Super-Properties: eyJvcyI6ImlPUyIsImJyb3dzZXIiOiJEaXNjb3JkIGlPUyIsImRldmljZSI6ImlQaG9uZTksMyIsInN5c3RlbV9sb2NhbGUiOiJlbi1DQSIsImNsaWVudF92ZXJzaW9uIjoiMTcyLjAiLCJyZWxlYXNlX2NoYW5uZWwiOiJzdGFibGUiLCJicm93c2VyX3VzZXJfYWdlbnQiOiIiLCJicm93c2VyX3ZlcnNpb24iOiIiLCJvc192ZXJzaW9uIjoiMTUuNSIsImNsaWVudF9idWlsZF9udW1iZXIiOjQyNjU2LCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsLCJkZXNpZ25faWQiOjB9' \
	--data-binary $'{\"content\":\"\",\"channel_id\":\"'$chan$'\",\"type\":0,\"flags\":8192,\"attachments\":[{\"id\":\"0\",\"filename\":\"voice-message.ogg\",\"uploaded_filename\":\"'$fn$'\",\"duration_secs\":'`soxi -D $file`$',\"waveform\":\"'`dd if=/dev/urandom bs=57 count=1 | base64`'"}]}' \
	$'https://discord.com/api/v9/channels/'$chan'/messages'