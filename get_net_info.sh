curl 'http://192.168.0.1/xml_action.cgi?method=set' \
  -H 'Accept: application/xml, text/xml, */*' \
  -H 'Accept-Language: ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7' \
  -H 'Authorization: Digest username="admin", realm="Highwmg", nonce="1000", uri="/cgi/xml_action.cgi", response="61f31cc4199d4e65e3a131f1f8f387ea", qop=auth, nc=00000037, cnonce="4254c0f3f26845b4"' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -H 'Cookie: locale=en; {auth_cookie}' \
  -H 'Origin: http://192.168.0.1' \
  -H 'Referer: http://192.168.0.1/index.html' \
  -H 'User-Agent: Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36' \
  -H 'X-Requested-With: XMLHttpRequest' \
  --data-raw '<?xml version="1.0" encoding="US-ASCII"?> <RGW><param><method>call</method><session>000</session><obj_path>cm</obj_path><obj_method>get_link_context</obj_method></param></RGW>' \
  --compressed \
  --insecure