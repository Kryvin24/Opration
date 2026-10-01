import zipfile
OV = 'id="termOverlay" hidden'
OT = 'id="termOverlay"'
TC = 'id="termContainer"'
RB = 'id="resourceBody"'
XJ = 'vendor/xterm.js'
for v in ('0.1.6','0.1.7','0.1.8','0.1.9'):
    p = rf'e:\DBX\dist\io.zdiai.teleport-{v}-windows-x64.dbxp'
    with zipfile.ZipFile(p) as z:
        html = z.read('ui/index.html').decode('utf-8', errors='replace')
    print(f"== {v}: len={len(html)}")
    print("   overlayHidden=", OV in html, " overlayTag=", OT in html, " termContainer=", TC in html)
    print("   resourceBody=", RB in html, " xtermJs=", XJ in html, " h1Teleport=", ('<h1>' in html and 'Teleport' in html))
    body = html.find('<body>')
    print("   body-start:", html[body:body+280].replace('\n',' '))
