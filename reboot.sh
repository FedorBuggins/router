curl 'http://192.168.0.1/xml_action.cgi?method=set' \
  -H 'Accept: application/xml, text/xml, */*' \
  -H 'Accept-Language: ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7' \
  -H 'Authorization: Digest username="admin", realm="Highwmg", nonce="1000", uri="/cgi/xml_action.cgi", response="d9ea4b3d0ddef666b765826742f5ee79", qop=auth, nc=0000000C, cnonce="ac43b12ba765cbe6"' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -H 'Cookie: locale=en; {auth_cookie}' \
  -H 'Origin: http://192.168.0.1' \
  -H 'Referer: http://192.168.0.1/index.html' \
  -H 'User-Agent: Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36' \
  -H 'X-Requested-With: XMLHttpRequest' \
  --data-raw '<?xml version="1.0" encoding="US-ASCII"?> <RGW><param><method>call</method><session>000</session><obj_path>router</obj_path><obj_method>router_call_reboot</obj_method></param></RGW>' \
  --compressed \
  --insecure
