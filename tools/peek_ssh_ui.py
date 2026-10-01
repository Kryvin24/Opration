import re
p = r'c:\Users\zdiai\AppData\Roaming\com.dbx.app\plugins\io.dbx.ssh\versions\0.7.0\ui\index.html'
with open(p, encoding='utf-8', errors='replace') as f:
    html = f.read()
body = html[html.find('<body>'):]
print("BODY LEN:", len(body))
# strip tags to find visible text
text = re.sub(r'<script[\s\S]*?</script>', ' ', body)
text = re.sub(r'<style[\s\S]*?</style>', ' ', text)
text = re.sub(r'<[^>]+>', ' ', text)
text = re.sub(r'\s+', ' ', text)
print("VISIBLE TEXT (first 1500):")
print(text[:1500])
