import sqlite3, json
con = sqlite3.connect(r'file:C:\Users\zdiai\AppData\Roaming\com.dbx.app\dbx.db?mode=ro', uri=True)
cur = con.cursor()

print("=== connections ===")
for rid, cj in cur.execute("select id, config_json from connections"):
    try:
        cfg = json.loads(cj)
        print(rid, "->", json.dumps({k: cfg.get(k) for k in ('name','db_type','plugin_id','plugin_connection_provider','plugin_connection_type','driver_profile','driver_label','host','port','username','password','external_config')}, ensure_ascii=False)[:400])
    except Exception as e:
        print(rid, "raw", cj[:200], e)

print("\n=== app_state ===")
for key, vj in cur.execute("select key, value_json from app_state"):
    print(key, "->", vj[:3000])

print("\n=== app_settings ===")
for rid, sj in cur.execute("select id, settings_json from app_settings"):
    print(sj[:3000])
con.close()
