curl 'http://192.168.0.1/login.cgi?Action=Digest&username=admin&realm=Highwmg&nonce=1000&response=3967b1186ffb8c7a7eda60b03f86b570&qop=auth&cnonce=37caec379f408a17&nc=00000001&temp=marvell&_=1706096581610' \
  -H 'Accept: */*' \
  -H 'Accept-Language: ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7' \
  -H 'Authorization: Digest username="admin", realm="Highwmg", nonce="1000", uri="/cgi/xml_action.cgi", response="2b6fec976abd4b413a8b837bf1b58cbc", qop=auth, nc=00000001, cnonce="94e46a62d315c5bf"' \
  -H 'Cache-Control: no-store, no-cache, must-revalidate' \
  -H 'Connection: keep-alive' \
  -H 'Cookie: locale=en' \
  -H 'Expires: -1' \
  -H 'Pragma: no-cache' \
  -H 'Referer: http://192.168.0.1/index.html' \
  -H 'User-Agent: Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36' \
  -H 'X-Requested-With: XMLHttpRequest' \
  --compressed \
  --insecure \
  -D -
