# DBX Teleport 插件 - Git 同步脚本
# 用法：右键「使用 PowerShell 运行」，或在终端执行：
#   powershell -ExecutionPolicy Bypass -File e:\DBX\tools\git-sync.ps1
#
# 仓库布局：
#   E:\DBX     -> origin/master  (Windows 版主线)
#   E:\DBX-mac -> origin/mac     (macOS 适配分支)

$GitBin = "C:\Program Files\Git\cmd"
$env:PATH = "$GitBin;$env:PATH"

$WinRepo = "E:\DBX"
$MacRepo = "E:\DBX-mac"

function Invoke-Git {
    param([string]$Repo, [string[]]$Args)
    Write-Host "`n> git -C $Repo $Args" -ForegroundColor DarkGray
    & git -C $Repo @Args
    if ($LASTEXITCODE -ne 0) { throw "git 命令失败（退出码 $LASTEXITCODE）" }
}

function Show-Status {
    Write-Host "`n===== Windows 版（E:\DBX / master）=====" -ForegroundColor Cyan
    & git -C $WinRepo status --short --branch
    Write-Host "`n===== Mac 版（E:\DBX-mac / mac）=====" -ForegroundColor Cyan
    & git -C $MacRepo status --short --branch
}

function Save-Repo {
    param([string]$Repo, [string]$Branch, [string]$Message)
    $pending = & git -C $Repo status --porcelain
    if (-not $pending) {
        Write-Host "没有需要提交的改动。" -ForegroundColor Yellow
        return
    }
    Invoke-Git $Repo @("add", "-A")
    Invoke-Git $Repo @("commit", "-m", $Message)
    Invoke-Git $Repo @("push", "origin", "${Branch}:${Branch}")
    Write-Host "已推送到 origin/$Branch" -ForegroundColor Green
}

function Sync-MacFromMaster {
    # 把 Windows 主线的新提交合并进 mac 分支（Mac 版仓库内操作）
    Invoke-Git $MacRepo @("fetch", "origin", "master")
    Invoke-Git $MacRepo @("merge", "origin/master", "--no-edit")
    Invoke-Git $MacRepo @("push", "origin", "master:mac")
    Write-Host "master 已合并进 mac 分支。" -ForegroundColor Green
}

while ($true) {
    Write-Host "`n========== DBX Git 同步 ==========" -ForegroundColor White
    Write-Host " 1) 查看两个仓库状态"
    Write-Host " 2) 提交并推送 Windows 版 (master)"
    Write-Host " 3) 提交并推送 Mac 版 (mac)"
    Write-Host " 4) 把 master 新改动合并进 mac 分支"
    Write-Host " 5) 拉取远端最新代码（两个仓库）"
    Write-Host " 0) 退出"
    $choice = Read-Host "`n请选择"
    try {
        switch ($choice) {
            "1" { Show-Status }
            "2" {
                $msg = Read-Host "提交信息"
                if ($msg) { Save-Repo $WinRepo "master" $msg }
            }
            "3" {
                $msg = Read-Host "提交信息"
                if ($msg) { Save-Repo $MacRepo "mac" $msg }
            }
            "4" { Sync-MacFromMaster }
            "5" {
                Invoke-Git $WinRepo @("pull", "--ff-only")
                Invoke-Git $MacRepo @("pull", "--ff-only")
            }
            "0" { return }
            default { Write-Host "无效选项" -ForegroundColor Yellow }
        }
    } catch {
        Write-Host "`n出错：$_" -ForegroundColor Red
    }
}
