    const I18N = {
      "zh-cn": {
        cluster: "集群", proxy: "代理", expiry: "证书有效期至",
        refresh: "刷新", logout: "登出",
        resources: "资源", fAll: "全部", fNodes: "节点", fApps: "应用", fKube: "K8s",
        loading: "正在加载…", empty: "暂无资源", noMatch: "无匹配资源", noSession: "无活动会话，请在连接上点 “tsh login” 登录。",
        searchPlaceholder: "搜索资源…",
        connected: "已连接", disconnected: "未连接", error: "错误",
        loginHint: "登录请在左侧连接上点 “tsh login”（每次都会重新收集验证码，不保存 OTP）。",
        expiresIn: "将于 {t} 后过期", expired: "已过期 {t}",
        ssh: "SSH", forward: "转发", kubeLogin: "注入上下文", kubeEnabled: "Kube 可用",
        kindNode: "SSH Server", kindApp: "App", kindKube: "Kubernetes",
        probe: "探活", probing: "探活中", confirming: "慢链路确认中 {n}", batch: "批量执行", clear: "清空", run: "执行",
        slowBadge: "慢", slowHint: "响应耗时 {s}s",
        selected: "已选 {n} 个节点",
        concurrency: "并发", timeoutSec: "超时(秒)",
        batchCommandPh: "输入要在所有选中节点执行的命令，如 systemctl status nginx",
        bColHost: "节点", bColStatus: "状态", bColDuration: "耗时",
        sPending: "等待", sOk: "成功", sFail: "失败", sTimeout: "超时", sError: "错误",
        bDone: "完成 {ok}/{total}，失败 {fail}",
        otherGroup: "其他资源",
        forwardTitle: "端口转发", fwdCreateTitle: "新建转发",
        fwdNode: "节点", fwdLocalPort: "本地端口", fwdTargetHost: "目标地址", fwdTargetPort: "目标端口",
        fwdStart: "启动转发", fwdTNode: "节点", fwdTMap: "转发映射", fwdTState: "状态", fwdTActions: "操作",
        fwdRunning: "运行中", fwdCopy: "复制地址", fwdStop: "停止",
        fwdCreated: "转发已启动：{url}", fwdNone: "暂无活动转发",
        kubeInject: "注入", kubeDone: "已注入上下文：{ctx}（kubeconfig: {path}）",
        snippets: "片段", add: "添加",
        autoReconnect: "自动重连", reconnect: "重连",
        connLost: "连接已断开", reconnecting: "正在重新连接…",
        reconnected: "已重新连接", reconnectFail: "重连失败：{msg}",
        retryHint: "{n}s 后自动重试（第 {attempt} 次）",
        autoStopped: "已达最大重试次数，请手动重连",
        snippetAddPh: "命令片段，以空格结尾则只插入不执行",
        report: "报告", copyReport: "复制报告", backToTable: "返回表格",
        inspect: "巡检", inspTitle: "巡检总览", inspRefresh: "刷新", inspAuto: "自动刷新（10分钟）",
        inspLast: "上次", inspNever: "尚未采集", inspRunning: "采集中…",
        inspOk: "正常", inspWarn: "偏高", inspBad: "告警", inspOff: "离线", inspPending: "待采集",
        inspDisk: "磁盘 /", inspMem: "内存", inspLoad: "负载(1m)", inspDocker: "容器", inspDuration: "耗时",
        inspOffline: "离线", inspFailed: "采集失败", inspNoDocker: "未安装 docker",
        rptTitle: "批量执行结果报告",
        rptCmd: "命令", rptTime: "生成时间",
        rptStat: "统计：共 {total} 台 · 成功 {ok} · 失败 {fail}",
        rptDur: "耗时：总 {totalMs}ms · 平均 {avgMs}ms · 最快 {minMs}ms · 最慢 {maxMs}ms",
        rptDetail: "节点明细",
        rptNoOutput: "（无输出）",
        transfer: "文件", transferTitle: "文件传输",
        xferUp: "上传到节点", xferDown: "从节点下载",
        xferUpTitle: "上传文件 / 目录", xferDownTitle: "下载文件 / 目录",
        xferLocal: "本地路径", xferRemote: "目标目录（节点上）", xferRecursive: "目录",
        xferStartUp: "开始上传", xferStartDown: "开始下载",
        xferDownRemote: "远端路径（节点上）", xferDownLocal: "保存到本地",
        xferPushAll: "同时推送到已勾选的 {n} 个节点",
        xferRunning: "传输中", xferUpDone: "全部完成：成功 {ok}/{total}",
        xferDownDone: "已下载到 {path}",
        xferNodeSingle: "只传到当前节点",
        diag: "诊断包", diagSaved: "诊断包已保存（路径已复制）："
      },
      en: {
        cluster: "Cluster", proxy: "Proxy", expiry: "Cert valid until",
        refresh: "Refresh", logout: "Logout",
        resources: "Resources", fAll: "All", fNodes: "Nodes", fApps: "Apps", fKube: "K8s",
        loading: "Loading…", empty: "No resources", noMatch: "No matching resources", noSession: "No active session — click “tsh login” on the connection.",
        searchPlaceholder: "Search resources…",
        connected: "Connected", disconnected: "Disconnected", error: "Error",
        loginHint: "To log in, click “tsh login” on the connection (OTP is collected fresh every time).",
        expiresIn: "expires in {t}", expired: "expired {t} ago",
        ssh: "SSH", forward: "Forward", kubeLogin: "Inject context", kubeEnabled: "Kube enabled",
        kindNode: "SSH Server", kindApp: "App", kindKube: "Kubernetes",
        probe: "Probe", probing: "Probing", confirming: "Confirming {n}", batch: "Batch run", clear: "Clear", run: "Run",
        slowBadge: "slow", slowHint: "response took {s}s",
        selected: "{n} nodes selected",
        concurrency: "Concurrency", timeoutSec: "Timeout(s)",
        batchCommandPh: "Command to run on every selected node, e.g. systemctl status nginx",
        bColHost: "Node", bColStatus: "Status", bColDuration: "Duration",
        sPending: "Pending", sOk: "OK", sFail: "Failed", sTimeout: "Timeout", sError: "Error",
        bDone: "Done {ok}/{total}, {fail} failed",
        otherGroup: "Other resources",
        forwardTitle: "Port forwarding", fwdCreateTitle: "New forward",
        fwdNode: "Node", fwdLocalPort: "Local port", fwdTargetHost: "Target host", fwdTargetPort: "Target port",
        fwdStart: "Start forward", fwdTNode: "Node", fwdTMap: "Mapping", fwdTState: "State", fwdTActions: "Actions",
        fwdRunning: "running", fwdCopy: "Copy URL", fwdStop: "Stop",
        fwdCreated: "Forward started: {url}", fwdNone: "No active forwards",
        kubeInject: "Inject", kubeDone: "Context injected: {ctx} (kubeconfig: {path})",
        snippets: "Snippets", add: "Add",
        autoReconnect: "Auto reconnect", reconnect: "Reconnect",
        connLost: "Connection lost", reconnecting: "Reconnecting…",
        reconnected: "Reconnected", reconnectFail: "Reconnect failed: {msg}",
        retryHint: "auto retry in {n}s (attempt {attempt})",
        autoStopped: "max retries reached, please reconnect manually",
        snippetAddPh: "Command snippet; trailing space inserts without running",
        report: "Report", copyReport: "Copy report", backToTable: "Back to table",
        inspect: "Inspect", inspTitle: "Fleet inspection", inspRefresh: "Refresh", inspAuto: "Auto refresh (10 min)",
        inspLast: "Last", inspNever: "Not collected yet", inspRunning: "Collecting…",
        inspOk: "OK", inspWarn: "Warning", inspBad: "Alert", inspOff: "Offline", inspPending: "Pending",
        inspDisk: "Disk /", inspMem: "Memory", inspLoad: "Load (1m)", inspDocker: "Containers", inspDuration: "Duration",
        inspOffline: "offline", inspFailed: "collect failed", inspNoDocker: "no docker",
        rptTitle: "Batch execution report",
        rptCmd: "Command", rptTime: "Generated",
        rptStat: "Stats: {total} total · {ok} ok · {fail} failed",
        rptDur: "Duration: total {totalMs}ms · avg {avgMs}ms · min {minMs}ms · max {maxMs}ms",
        rptDetail: "Per-node detail",
        rptNoOutput: "(no output)",
        transfer: "Files", transferTitle: "File transfer",
        xferUp: "Upload", xferDown: "Download",
        xferUpTitle: "Upload file / directory", xferDownTitle: "Download file / directory",
        xferLocal: "Local path", xferRemote: "Destination dir (on node)", xferRecursive: "Directory",
        xferStartUp: "Start upload", xferStartDown: "Start download",
        xferDownRemote: "Remote path (on node)", xferDownLocal: "Save locally to",
        xferPushAll: "Also push to {n} selected nodes",
        xferRunning: "transferring", xferUpDone: "All done: {ok}/{total} ok",
        xferDownDone: "Downloaded to {path}",
        xferNodeSingle: "Current node only",
        diag: "Diag bundle", diagSaved: "Diagnostics saved (path copied): "
      }
    };

    let t = I18N["zh-cn"];
    let connectionId = null;
    let filter = "all";
    let search = "";
    let resources = [];
    let busy = false;

    // ---- Batch execution / liveness state ----
    let selectedHosts = new Set();    // checked node hostnames
    let onlineMap = new Map();        // hostname -> reachable (after probe)
    let slowHosts = new Map();        // hostname -> best duration ms for slow link
    let probed = false;
    let collapsedGroups = new Set();  // collapsed group names
    let activeBatchId = null;
    let activeProbeId = null;
    let batchRows = new Map();        // hostname -> result row
    let batchExpanded = new Set();    // hostnames with output expanded
    let batchRunning = false;

    const $ = (sel) => document.querySelector(sel);

    function applyI18n() {
      document.querySelectorAll("[data-i18n]").forEach((el) => {
        const key = el.dataset.i18n;
        if (t[key]) el.textContent = t[key];
      });
      document.querySelectorAll("[data-i18n-placeholder]").forEach((el) => {
        const key = el.dataset.i18nPlaceholder;
        if (t[key]) el.placeholder = t[key];
      });
    }

    function fmtDuration(ms) {
      const s = Math.abs(Math.floor(ms / 1000));
      const h = Math.floor(s / 3600), m = Math.floor((s % 3600) / 60);
      if (h >= 24) {
        const d = Math.floor(h / 24);
        return `${d}d ${h % 24}h`;
      }
      if (h > 0) return `${h}h ${m}m`;
      return `${m}m`;
    }

    function setBadge(state) {
      const badge = $("#stateBadge");
      badge.className = `badge ${state}`;
      const map = { connected: t.connected, disconnected: t.disconnected, error: t.error };
      $("#stateText").textContent = map[state] || state;
    }

    function renderStatus(status) {
      if (!status) {
        setBadge("disconnected");
        $("#mCluster").textContent = "—";
        $("#mProxy").textContent = "—";
        $("#mExpiry").textContent = "—";
        return;
      }
      setBadge("connected");
      const sObj = status.status || status;
      $("#mCluster").textContent = sObj.cluster || status.cluster || "—";
      $("#mProxy").textContent = sObj.proxy_url || status.proxy_url || "—";
      const expiry = sObj.valid_until || sObj.expires || status.expires || "";
      if (expiry) {
        const ms = new Date(expiry).getTime() - Date.now();
        const rel = ms > 0 ? t.expiresIn.replace("{t}", fmtDuration(ms)) : t.expired.replace("{t}", fmtDuration(ms));
        $("#mExpiry").textContent = `${expiry} (${rel})`;
      } else {
        $("#mExpiry").textContent = "—";
      }
    }

    const KIND_SVG = {
      node: '<svg class="kind-ico" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="4" width="18" height="7" rx="1.5"/><rect x="3" y="13" width="18" height="7" rx="1.5"/><path d="M7 7.5h.01M7 16.5h.01"/></svg>',
      app: '<svg class="kind-ico" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="4" y="4" width="16" height="16" rx="3"/><path d="M9 9h6M9 13h4"/></svg>',
      kube: '<svg class="kind-ico" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M12 3l8 4.5v9L12 21l-8-4.5v-9L12 3z"/><path d="M12 12l8-4.5M12 12v9M12 12L4 7.5"/></svg>'
    };

    function actionsFor(kind) {
      if (kind === "node") {
        return [
          { act: "ssh", label: t.ssh },
          { act: "files", label: t.transfer },
          { act: "forward", label: t.forwardTitle },
        ];
      }
      if (kind === "kube") return [{ act: "kube", label: t.kubeInject }];
      return [];
    }

    // ===== Smart family grouping =====
    // A node "family" is the meaningful letter prefix shared by related ships:
    //   NewAchievement / NewFrontier  -> "New"
    //   LianHe5 / LianHe11            -> "LianHe" (trailing serial dropped)
    //   CspcAries / CspcLeo           -> "Cspc"
    // Discovery is fully data-driven — no family list is hard-coded:
    //   1) take the substring before the first "-"
    //   2) strip a trailing numeric serial (LianHe11 -> LianHe)
    //   3) merge canons sharing a prefix that ends on a CamelCase boundary
    //      (the next char must be uppercase), taking the LONGEST such prefix.
    // The boundary rule prevents false merges like "Ch" (CheraAmity vs
    // ChinaChampion) or "Xi"; requiring a LEADING prefix means
    // ShanDongNewEra never joins the "New" family.
    function rawPrefix(name) {
      const i = (name || "").indexOf("-");
      return i > 0 ? name.slice(0, i) : (name || "");
    }
    function stripTrailingDigits(s) {
      return s.replace(/\d+$/, "");
    }

    /// Build hostname -> family-group map for the given node resources.
    function buildHostGroupMap(nodes) {
      const canonOf = new Map();
      const canons = [];
      for (const r of nodes) {
        const canon = stripTrailingDigits(rawPrefix(r.name));
        canonOf.set(r.name, canon);
        if (!canons.includes(canon)) canons.push(canon);
      }

      // Family of a canon = longest prefix P shared with some OTHER canon where
      // the char right after P is uppercase (a CamelCase word boundary).
      function familyOf(canon) {
        let best = null;
        for (const other of canons) {
          if (other === canon) continue;
          const max = Math.min(canon.length, other.length);
          let p = 0;
          while (p < max && canon[p] === other[p]) p++;
          if (p >= 2 && p < canon.length) {
            const ch = canon[p];
            if (ch >= "A" && ch <= "Z" && (best === null || p > best.length)) {
              best = canon.slice(0, p);
            }
          }
        }
        return best === null ? canon : best;
      }

      const map = new Map();
      for (const r of nodes) map.set(r.name, familyOf(canonOf.get(r.name)));
      return map;
    }

    let hostGroupMap = new Map();
    function groupKey(r) {
      if (r.kind === "node") {
        return (
          hostGroupMap.get(r.name) ||
          stripTrailingDigits(rawPrefix(r.name)) ||
          t.otherGroup
        );
      }
      return t.otherGroup;
    }

    /// Tooltip for a slow-link badge (value is a backend duration in ms).
    function slowTitle(r) {
      const d = slowHosts.get(r.name);
      if (d == null) return "";
      return (t.slowHint || "响应耗时 {s}s").replace(
        "{s}",
        (d / 1000).toFixed(1)
      );
    }

    function renderResources() {
      const body = $("#resourceBody");
      // innerHTML rebuild snaps scroll to top; preserve it across re-renders
      // (probe results, batch ticks, search) so a long node list stays put.
      const savedScrollTop = body.scrollTop;
      // Family index is computed from every node (not the filtered list) so a
      // node's group never changes while searching or toggling kind filters.
      hostGroupMap = buildHostGroupMap(resources.filter((r) => r.kind === "node"));
      const q = search.trim().toLowerCase();
      const list = resources.filter((r) => {
        if (filter !== "all" && r.kind !== filter) return false;
        if (!q) return true;
        const haystack = [r.name, r.address, r.kind]
          .concat(r.labels && typeof r.labels === "object"
            ? Object.entries(r.labels).map(([k, v]) => `${k}: ${v}`)
            : [])
          .join(" ")
          .toLowerCase();
        return haystack.includes(q);
      });
      $("#resCount").textContent = q ? `${list.length} / ${resources.length}` : String(resources.length);
      if (list.length === 0) {
        body.innerHTML = `<div class="empty">${q ? t.noMatch : t.empty}</div>`;
        return;
      }

      // Bucket into ordered groups.
      const groupItems = new Map();
      for (const r of list) {
        const gk = groupKey(r);
        if (!groupItems.has(gk)) groupItems.set(gk, []);
        groupItems.get(gk).push(r);
      }
      const groupNames = [...groupItems.keys()]
        .sort((a, b) => a.localeCompare(b));
      // Other resources (apps/kube) float to the end.
      if (groupNames.includes(t.otherGroup)) {
        groupNames.splice(groupNames.indexOf(t.otherGroup), 1);
        groupNames.push(t.otherGroup);
      }

      // Flat index backing the card checkboxes (event handlers close over it).
      const cardList = [];
      const kindLabel = { node: t.kindNode, app: t.kindApp, kube: t.kindKube };
      __groupSlot = 0;

      const sectionsHtml = groupNames.map((gk) => {
        const items = groupItems.get(gk);
        const nodeItems = items.filter((r) => r.kind === "node");
        let countHtml;
        if (probed && nodeItems.length) {
          const down = nodeItems.filter((r) => onlineMap.get(r.name) === false).length;
          const up = nodeItems.length - down;
          countHtml = down
            ? `<span class="group-count">${up}/${nodeItems.length} <span class="down">↓${down}</span></span>`
            : `<span class="group-count">${up}/${nodeItems.length}</span>`;
        } else {
          countHtml = `<span class="group-count">${nodeItems.length || items.length}</span>`;
        }

        // Group checkbox state over visible, selectable (online) nodes only.
        const selectable = nodeItems.filter(
          (r) => !probed || onlineMap.get(r.name) !== false);
        const allSel = selectable.length > 0 &&
          selectable.every((r) => selectedHosts.has(r.name));

        const cards = items.map((r) => {
          const idx = cardList.length;
          cardList.push(r);
          const labels = r.labels && typeof r.labels === "object"
            ? `<div class="card-tags">${Object.entries(r.labels)
                .filter(([k, v]) => String(v) !== (r.name || ""))
                .slice(0, 3)
                .map(([k, v]) => `<span class="tag">${k}: ${v}</span>`).join("")}</div>`
            : "";
          const acts = actionsFor(r.kind);
          const actions = acts.length
            ? `<div class="card-actions">
                ${acts.map((a) =>
                  `<button type="button" data-act="${a.act}" data-card-idx="${idx}">${a.label}</button>`
                ).join("")}
              </div>`
            : "";
          const isNode = r.kind === "node";
          const offline = isNode && probed && onlineMap.get(r.name) === false;
          const slow = !offline && isNode && probed && slowHosts.has(r.name);
          const check = isNode
            ? `<label class="card-check"><input type="checkbox" data-card-check="${idx}" ${offline ? "disabled" : ""}></label>`
            : "";
          return `<div class="res-card${offline ? " offline" : ""}">
            <div class="card-top">
              ${check}
              ${KIND_SVG[r.kind] || ""}
              <div class="card-name">
                <div class="name" data-name-idx="${idx}"></div>
                <span class="kind-tag">${kindLabel[r.kind] || r.kind}</span>${offline ? `<span class="offline-badge">${t.sFail === "失败" ? "离线" : "offline"}</span>` : ""}${slow ? `<span class="slow-badge" title="${slowTitle(r)}">${t.slowBadge}</span>` : ""}
              </div>
            </div>
            ${labels}
            ${actions}
          </div>`;
        }).join("");

        return `<section class="group-section${collapsedGroups.has(gk) ? " collapsed" : ""}">
          <div class="group-head" data-group-head="${gk}">
            <span class="group-caret">${collapsedGroups.has(gk) ? "▶" : "▼"}</span>
            <span class="group-title" data-group-title="${idx_group(gk)}"></span>
            ${countHtml}
            ${selectable.length
              ? `<label class="group-check" data-group-check-label><input type="checkbox" data-group-select="${gk}" ${allSel ? "checked" : ""}></label>`
              : ""}
          </div>
          <div class="resource-grid group-grid">${cards}</div>
        </section>`;
      }).join("");

      body.innerHTML = sectionsHtml;

      // Group titles via textContent (group names come from tsh data).
      body.querySelectorAll("[data-group-title]").forEach((el) => {
        el.textContent = groupNames[Number(el.dataset.groupTitle)];
      });
      // Node names via textContent to avoid HTML injection.
      body.querySelectorAll("[data-name-idx]").forEach((el) => {
        const r = cardList[Number(el.dataset.nameIdx)];
        el.textContent = r.name || "—";
        el.title = r.name || "";
      });
      // Card checkboxes.
      body.querySelectorAll("input[data-card-check]").forEach((input) => {
        const r = cardList[Number(input.dataset.cardCheck)];
        input.checked = selectedHosts.has(r.name);
        input.addEventListener("click", (e) => e.stopPropagation());
        input.addEventListener("change", () => {
          if (input.checked) selectedHosts.add(r.name);
          else selectedHosts.delete(r.name);
          updateBatchBar();
          // Reflect the group checkbox without a full re-render.
          syncGroupCheckbox(body, r);
        });
      });
      // Collapse toggles.
      body.querySelectorAll(".group-head").forEach((head) => {
        head.addEventListener("click", (e) => {
          if (e.target.closest("input, .group-check")) return;
          const gk = head.dataset.groupHead;
          if (collapsedGroups.has(gk)) collapsedGroups.delete(gk);
          else collapsedGroups.add(gk);
          renderResources();
        });
      });
      // Group-wide select.
      body.querySelectorAll("input[data-group-select]").forEach((input) => {
        input.addEventListener("click", (e) => e.stopPropagation());
        input.addEventListener("change", () => {
          const gk = input.dataset.groupSelect;
          groupItems
            .get(gk)
            .filter((r) => r.kind === "node" && (!probed || onlineMap.get(r.name) !== false))
            .forEach((r) => {
              if (input.checked) selectedHosts.add(r.name);
              else selectedHosts.delete(r.name);
            });
          renderResources();
          updateBatchBar();
        });
      });
      // Action buttons.
      body.querySelectorAll("button[data-act]").forEach((btn) => {
        btn.addEventListener("click", () => {
          const r = cardList[Number(btn.dataset.cardIdx)];
          runCardAction(btn.dataset.act, r);
        });
      });
      body.scrollTop = savedScrollTop;
    }

    /// Encode a group name into a safe numeric slot index (the data attribute
    /// must stay injection-free); we only need identity within this render.
    let __groupSlot = 0;
    function idx_group(_gk) { return __groupSlot++; }

    /// Recompute one group's checkbox state after a card toggle.
    function syncGroupCheckbox(body, changed) {
      const gk = groupKey(changed);
      const input = body.querySelector(`input[data-group-select="${gk}"]`);
      if (!input) return;
      // Count the rendered, selectable cards belonging to this family.
      const checks = body.querySelectorAll("input[data-card-check]");
      let total = 0, checked = 0;
      checks.forEach((c) => {
        if (c.disabled) return;
        const nameEl = c.closest(".res-card").querySelector("[data-name-idx]");
        if (nameEl && hostGroupMap.get(nameEl.textContent) === gk) {
          total++;
          if (c.checked) checked++;
        }
      });
      input.checked = total > 0 && checked === total;
    }

    async function callSidecar(method, params, timeoutMs) {
      return window.dbxPlugin.invoke(method, params, timeoutMs ? { timeoutMs } : {});
    }

    async function refresh() {
      if (busy) return;
      busy = true;
      $("#btnRefresh").disabled = true;
      try {
        const [statusRes, listRes] = await Promise.all([
          callSidecar("connection/action", { connectionId, action: { id: "refresh" } }, 15000)
            .catch((e) => ({ error: String(e.message || e) })),
          callSidecar("teleport/listResources", { connectionId }, 15000)
            .catch((e) => ({ success: false, resources: [], message: String(e.message || e) }))
        ]);

        const hasStatus = statusRes && statusRes.success && statusRes.status;
        renderStatus(hasStatus ? statusRes : null);

        const lr = listRes || {};
        resources = Array.isArray(lr.resources) ? lr.resources : [];
        renderResources();

        const hint = $("#hint");
        if (resources.length === 0 && lr.message) {
          // Diagnostic: surface the backend message so we can see why the list
          // is empty (expired cert vs tsh subprocess failure vs empty result).
          hint.hidden = false;
          hint.textContent = `listResources: ${lr.message}`;
        } else if (!hasStatus) {
          hint.hidden = false;
          hint.textContent = t.loginHint;
        } else {
          hint.hidden = true;
        }
      } catch (e) {
        const hint = $("#hint");
        hint.hidden = false;
        hint.textContent = `refresh error: ${String((e && e.message) || e)}`;
      } finally {
        busy = false;
        $("#btnRefresh").disabled = false;
      }
    }

    // -----------------------------------------------------------------------
    // Tab system (browser-like multi-panel tabs)
    // -----------------------------------------------------------------------
    const TAB_HOME = "home";       // 首页标签：资源列表，常驻且不可关闭
    const HOME_TAB_ID = "tab-home";
    const TAB_SSH = "ssh";
    const TAB_BATCH = "batch";
    const TAB_FORWARD = "forward";
    const TAB_TRANSFER = "transfer";
    const TAB_INSPECT = "inspect";
    const SINGLETON_TABS = new Set([TAB_BATCH, TAB_FORWARD, TAB_TRANSFER, TAB_INSPECT]);

    // Map overlay element id -> tab type
    const OVERLAY_FOR_TYPE = {
      [TAB_BATCH]: "batchOverlay",
      [TAB_FORWARD]: "forwardOverlay",
      [TAB_TRANSFER]: "transferOverlay",
      [TAB_INSPECT]: "inspectOverlay",
    };
    const TAB_TITLE = {
      [TAB_BATCH]: "批量执行",
      [TAB_FORWARD]: "端口转发",
      [TAB_TRANSFER]: "文件传输",
      [TAB_INSPECT]: "巡检总览",
    };

    const openTabs = [];          // [{id, type, title, sessionId?, nodeName?}]
    let activeTabId = null;

    function renderTabBar() {
      const bar = $("#tabBar");
      bar.innerHTML = "";
      if (openTabs.length === 0) { bar.hidden = true; return; }
      bar.hidden = false;
      openTabs.forEach((tab) => {
        const item = document.createElement("div");
        item.className = "tab-item" + (tab.id === activeTabId ? " active" : "");
        item.dataset.tabId = tab.id;
        if (tab.type === TAB_HOME) {
          const ico = document.createElement("span");
          ico.className = "tab-ico";
          ico.innerHTML = '<svg viewBox="0 0 24 24" width="13" height="13" aria-hidden="true"><path d="M12 3.4 3.5 11h2.3v8.6h4.6v-5.4h3.2v5.4h4.6V11h2.3z" fill="currentColor"/></svg>';
          item.appendChild(ico);
        }
        const title = document.createElement("span");
        title.className = "tab-title";
        title.textContent = tab.title;
        item.appendChild(title);
        if (tab.type !== TAB_HOME) {
          // 首页标签常驻，不提供关闭按钮
          const closeBtn = document.createElement("button");
          closeBtn.className = "tab-close";
          closeBtn.type = "button";
          closeBtn.textContent = "✕";
          closeBtn.title = "关闭";
          closeBtn.addEventListener("click", (e) => { e.stopPropagation(); closeTab(tab.id); });
          item.appendChild(closeBtn);
        }
        item.addEventListener("click", () => activateTab(tab.id));
        bar.appendChild(item);
      });
    }

    /// 资源列表即“首页”标签：常驻、不可关闭、始终排在最前。
    function ensureHomeTab() {
      let home = openTabs.find((t) => t.type === TAB_HOME);
      if (!home) {
        home = { id: HOME_TAB_ID, type: TAB_HOME, title: "首页" };
        openTabs.unshift(home);
      }
      return home;
    }

    function showResourcePanel() {
      // 资源列表 = 首页标签，激活它即可。
      activateTab(ensureHomeTab().id);
    }

    function activateTab(id) {
      const tab = openTabs.find((t) => t.id === id);
      if (!tab) { showResourcePanel(); return; }
      activeTabId = id;
      const isHome = tab.type === TAB_HOME;
      // Hide everything first.
      document.querySelectorAll(".batch-overlay, .term-overlay").forEach((el) => el.classList.remove("open"));
      if (isHome) {
        // 首页视图（头部信息 + 资源列表）整体属于首页标签，在标签栏下方显示
        $("#homeView").style.display = "";
        $("#batchBar").style.display = "";
      } else {
        // 功能标签：隐藏整个首页视图，由对应面板铺满内容区
        $("#homeView").style.display = "none";
        $("#batchBar").style.display = "none";
        const overlayId = OVERLAY_FOR_TYPE[tab.type];
        if (overlayId) {
          document.getElementById(overlayId)?.classList.add("open");
        } else if (tab.type === TAB_SSH) {
          // Terminal overlay is stored on the session object.
          if (tab.session && tab.session.el) tab.session.el.classList.add("open");
        }
      }
      renderTabBar();
    }

    function openTab(type, title, opts) {
      // Singleton tabs: if already open, just activate it.
      if (SINGLETON_TABS.has(type)) {
        const existing = openTabs.find((t) => t.type === type);
        if (existing) { activateTab(existing.id); return existing.id; }
      }
      const id = "tab-" + Date.now() + "-" + Math.random().toString(36).slice(2, 7);
      const tab = { id, type, title: title || TAB_TITLE[type] || type, ...(opts || {}) };
      openTabs.push(tab);
      activateTab(id);
      return id;
    }

    function closeTab(id) {
      const idx = openTabs.findIndex((t) => t.id === id);
      if (idx === -1) return;
      const tab = openTabs[idx];
      if (tab.type === TAB_HOME) return; // 首页常驻，不可关闭
      // Cleanup per-type.
      if (tab.type === TAB_SSH && tab.session) {
        tab.session.destroy();
      }
      openTabs.splice(idx, 1);
      const next = openTabs[idx] || openTabs[idx - 1];
      if (next) activateTab(next.id);
      else { activeTabId = null; showResourcePanel(); renderTabBar(); }
    }

    // -----------------------------------------------------------------------
    // SSH terminal (xterm.js — multi-instance, one per tab)
    // -----------------------------------------------------------------------
    // Each SSH tab owns a TerminalSession: its own xterm, DOM overlay, session
    // id, reconnect state. Frames are routed by session id so multiple terminals
    // can live side by side.
    const terminalSessions = new Map(); // sid -> TerminalSession
    const pendingOut = new Map();        // sid -> buffered frames before xterm opens

    // Terminal font size, persisted across sessions (10..24px).
    let termFontSize = 13;
    try {
      const v = parseInt(localStorage.getItem("teleport.termFontSize"), 10);
      if (v >= 10 && v <= 24) termFontSize = v;
    } catch (e) {}

    function setTermFontSize(px) {
      termFontSize = Math.min(24, Math.max(10, px));
      try { localStorage.setItem("teleport.termFontSize", String(termFontSize)); } catch (e) {}
      terminalSessions.forEach((s) => s.setFontSize(termFontSize));
    }

    function isTerminalChannel(ch) {
      return typeof ch === "string" && ch.startsWith("ssh/terminal/out/");
    }

    function onTerminalFrame(sid, stream, seq, payload) {
      const sess = terminalSessions.get(sid);
      if (sess && sess.term && !sess.done) {
        sess.writeFrame(seq, stream, payload);
      } else {
        if (!pendingOut.has(sid)) pendingOut.set(sid, []);
        pendingOut.get(sid).push({ seq, stream, payload });
      }
    }

    function flushPendingFrames(sid) {
      const sess = terminalSessions.get(sid);
      if (!sess) return;
      const queued = pendingOut.get(sid) || [];
      pendingOut.delete(sid);
      queued.sort((a, b) => a.seq - b.seq).forEach(({ seq, stream, payload }) => {
        sess.writeFrame(seq, stream, payload);
      });
    }

    /// Create a new terminal session: clones the overlay template, wires xterm,
    /// returns the session object. Caller sets sessionId once the backend
    /// responds.
    function createTerminalSession(nodeName) {
      const tpl = $("#termOverlayTemplate");
      const frag = tpl.content.cloneNode(true);
      const el = frag.querySelector(".term-overlay");
      el.dataset.node = nodeName;
      $("#terminalContainer").appendChild(el);

      const sess = {
        el,
        nodeName,
        sessionId: null,
        term: null,
        done: false,
        lastSeq: 0,
        autoReconnect: false,
        reconnectTimer: null,
        reconnectAttempts: 0,
        reconnectInFlight: false,

        setFontSize(px) {
          if (this.term) {
            this.term.options.fontSize = px;
            try { this.fit(); this.sendResize(); } catch (e) {}
          }
        },

        fit() {
          if (!this.term) return;
          const container = el.querySelector(".term-container");
          if (!container) return;
          const width = container.clientWidth, height = container.clientHeight;
          if (!width || !height) return;
          const fontFamily = (this.term.options && this.term.options.fontFamily) || 'Consolas, "Cascadia Mono", monospace';
          const fontSize = (this.term.options && this.term.options.fontSize) || 13;
          let cw = Math.round(fontSize * 0.6), ch = Math.round(fontSize * 1.2);
          try {
            const probe = document.createElement("span");
            probe.textContent = "W";
            probe.setAttribute("aria-hidden", "true");
            probe.style.cssText = `position:absolute;visibility:hidden;top:0;left:0;white-space:pre;line-height:normal;font-family:${fontFamily};font-size:${fontSize}px;`;
            container.appendChild(probe);
            const rect = probe.getBoundingClientRect();
            if (rect && rect.width > 0 && rect.height > 0) { cw = rect.width; ch = rect.height; }
            container.removeChild(probe);
          } catch (e) {}
          this.term.resize(Math.max(2, Math.floor(width / cw)), Math.max(1, Math.floor(height / ch)));
        },

        sendResize() {
          if (!this.sessionId || this.done || !this.term) return;
          const cols = this.term.cols, rows = this.term.rows;
          if (!cols || !rows || isNaN(cols) || isNaN(rows)) return;
          window.dbxPlugin.notify("ssh/terminal/resize", { sessionId: this.sessionId, cols, rows }).catch(() => {});
        },

        writeFrame(seq, stream, payload) {
          if (seq > this.lastSeq) {
            this.lastSeq = seq;
            if (stream === 2) this.handleEnd();
            else { try { this.term.write(payload); } catch (e) {} }
          }
        },

        showError(msg) {
          const c = el.querySelector(".term-container");
          if (c) {
            c.style.display = "block"; c.style.padding = "14px 16px";
            c.style.fontFamily = "ui-monospace, Consolas, monospace"; c.style.fontSize = "13px";
            c.style.color = "#9c2f1f"; c.style.background = "#f6f1e7";
            c.textContent = `[term error] ${msg}`;
          }
          try { window.dbxPlugin.copy(String(msg)); } catch (e) {}
        },

        open() {
          try {
            if (!window.Terminal || typeof window.Terminal !== "function") throw new Error("xterm Terminal unavailable");
            this.term = new window.Terminal({
              cursorBlink: true, fontSize: termFontSize,
              fontFamily: 'Consolas, "Cascadia Mono", monospace',
              theme: { background: "#f6f1e7", foreground: "#3b3021", cursor: "#b5791f", selectionBackground: "#d9c9a3" },
              scrollback: 5000, allowTransparency: false,
            });
            this.term.open(el.querySelector(".term-container"));
            this.fit();
            this.term.focus();
            const self = this;
            this.term.onData((data) => {
              if (self.sessionId && !self.done) {
                let bytes;
                try { bytes = new TextEncoder().encode(data); } catch (e) { bytes = new Uint8Array(data.length); for (let i = 0; i < data.length; i++) bytes[i] = data.charCodeAt(i) & 0xff; }
                window.dbxPlugin.sendBinary(`ssh/terminal/in/${self.sessionId}`, bytes);
              }
            });
            this.term.onResize(() => self.sendResize());
            new ResizeObserver(() => { try { self.fit(); self.sendResize(); } catch (e) {} }).observe(el.querySelector(".term-container"));
          } catch (e) {
            this.term = null;
            this.showError(String((e && e.message) || e));
          }
        },

        handleEnd() {
          if (this.done) return;
          this.done = true;
          try { this.term.write("\r\n\x1b[90m[" + t.connLost + "]\x1b[0m\r\n"); } catch (e) {}
          this.reconnectAttempts = 0;
          const bar = el.querySelector(".reconnect-bar");
          if (bar) { bar.hidden = false; bar.querySelector(".reconnect-msg").textContent = t.connLost; }
          if (this.autoReconnect && this.nodeName) this.scheduleReconnect(2000, 1);
        },

        scheduleReconnect(delayMs, attempt) {
          clearTimeout(this.reconnectTimer);
          const bar = el.querySelector(".reconnect-bar");
          if (bar) {
            bar.hidden = false;
            bar.querySelector(".reconnect-msg").textContent = t.retryHint.replace("{n}", String(Math.round(delayMs / 1000))).replace("{attempt}", String(attempt));
          }
          const self = this;
          this.reconnectTimer = setTimeout(() => self.reconnect(), delayMs);
        },

        async reconnect() {
          if (this.reconnectInFlight || !this.nodeName) return;
          this.reconnectInFlight = true;
          clearTimeout(this.reconnectTimer);
          try { this.term.write("\x1b[33m[" + t.reconnecting + "]\x1b[0m\r\n"); } catch (e) {}
          try {
            const res = await callSidecar("contextMenu/io.zdiai.teleport.ssh", { connectionId, name: this.nodeName }, 25000);
            const sid = res && (res.sessionId || res.session_id);
            if (!sid) throw new Error("missing session id");
            // Drop the old session mapping, install the new one.
            if (this.sessionId) terminalSessions.delete(this.sessionId);
            this.sessionId = sid;
            terminalSessions.set(sid, this);
            this.done = false;
            this.lastSeq = 0;
            this.reconnectAttempts = 0;
            try { this.term.write("\x1b[32m[" + t.reconnected + "]\x1b[0m\r\n"); } catch (e) {}
            const bar = el.querySelector(".reconnect-bar");
            if (bar) bar.hidden = true;
            flushPendingFrames(sid);
            setTimeout(() => this.sendResize(), 80);
            try { this.term.focus(); } catch (e) {}
          } catch (e) {
            this.reconnectAttempts++;
            const msg = String((e && e.message) || e);
            try { this.term.write("\x1b[31m[" + t.reconnectFail.replace("{msg}", msg) + "]\x1b[0m\r\n"); } catch (er) {}
            if (this.autoReconnect && this.reconnectAttempts < 5) {
              const backoff = [3000, 5000, 8000, 10000, 10000][this.reconnectAttempts - 1] || 10000;
              this.scheduleReconnect(backoff, this.reconnectAttempts + 1);
            } else {
              const bar = el.querySelector(".reconnect-bar");
              if (bar) { bar.hidden = false; bar.querySelector(".reconnect-msg").textContent = this.autoReconnect ? t.autoStopped : t.reconnectFail.replace("{msg}", msg); }
            }
          } finally {
            this.reconnectInFlight = false;
          }
        },

        destroy() {
          clearTimeout(this.reconnectTimer);
          if (this.sessionId) {
            window.dbxPlugin.notify("ssh/terminal/close", { sessionId: this.sessionId }).catch(() => {});
            terminalSessions.delete(this.sessionId);
          }
          if (this.term) { try { this.term.dispose(); } catch (e) {} }
          el.remove();
        },
      };

      // Wire up the per-session controls.
      el.querySelector(".term-title").textContent = `SSH — ${nodeName}`;
      el.querySelector(".term-close").addEventListener("click", () => {
        const tab = openTabs.find((t) => t.session === sess);
        if (tab) closeTab(tab.id);
      });
      el.querySelector(".term-font-dec").addEventListener("click", () => setTermFontSize(termFontSize - 1));
      el.querySelector(".term-font-inc").addEventListener("click", () => setTermFontSize(termFontSize + 1));
      el.querySelector(".term-container").addEventListener("wheel", (e) => {
        if (!e.ctrlKey) return;
        e.preventDefault();
        setTermFontSize(termFontSize + (e.deltaY < 0 ? 1 : -1));
      }, { passive: false });
      el.querySelector(".reconnect-btn").addEventListener("click", () => { sess.reconnectAttempts = 0; sess.reconnect(); });
      el.querySelector(".auto-reconnect").addEventListener("change", (e) => {
        sess.autoReconnect = e.target.checked;
        if (sess.autoReconnect && sess.done && sess.nodeName && !sess.reconnectTimer) sess.scheduleReconnect(2000, 1);
        if (!sess.autoReconnect) clearTimeout(sess.reconnectTimer);
      });
      el.querySelector(".snippet-toggle").addEventListener("click", () => el.querySelector(".snippet-bar").classList.toggle("collapsed"));
      el.querySelector(".snippet-add-btn").addEventListener("click", () => {
        const row = el.querySelector(".snippet-add-row");
        row.hidden = !row.hidden;
        if (!row.hidden) { const inp = el.querySelector(".snippet-add-input"); inp.value = ""; inp.focus(); }
      });
      const confirmAdd = () => {
        const input = el.querySelector(".snippet-add-input");
        const v = input.value;
        if (v && !BUILTIN_SNIPPETS.includes(v) && !customSnippets.includes(v)) {
          customSnippets.push(v);
          persistCustomSnippets();
          renderAllSnippets();
        }
        input.value = "";
        el.querySelector(".snippet-add-row").hidden = true;
      };
      el.querySelector(".snippet-add-confirm").addEventListener("click", confirmAdd);
      el.querySelector(".snippet-add-input").addEventListener("keydown", (e) => { if (e.key === "Enter") confirmAdd(); });

      return sess;
    }

    /// Open an SSH terminal to the given node as a new tab. Multiple terminals
    /// to different (or the same) node can be open simultaneously.
    async function openSshTerminal(name) {
      if (!window.Terminal || typeof window.Terminal !== "function") {
        alert("xterm.js failed to load");
        return;
      }
      const sess = createTerminalSession(name);
      sess.open();
      renderSnippetsFor(sess);
      const tabId = openTab(TAB_SSH, `SSH — ${name}`, { session: sess });
      sess.tabId = tabId;

      let res;
      try {
        res = await callSidecar("contextMenu/io.zdiai.teleport.ssh", { connectionId, name }, 20000);
      } catch (e) {
        window.dbxPlugin.copy(String(e.message || e));
        sess.showError(String(e.message || e));
        return;
      }
      const sid = res && (res.sessionId || res.session_id);
      if (!sid) {
        window.dbxPlugin.copy(JSON.stringify(res));
        sess.showError("missing session id");
        return;
      }
      sess.sessionId = sid;
      terminalSessions.set(sid, sess);
      flushPendingFrames(sid);
      setTimeout(() => sess.sendResize(), 80);
    }

    // -----------------------------------------------------------------------
    // Quick command snippets
    // -----------------------------------------------------------------------
    const BUILTIN_SNIPPETS = [
      "uptime", "df -h", "free -m", "docker ps", "docker ps -a",
      "ss -tlnp", "ps aux", "ip addr", "last -n 10", "dmesg | tail -30",
      "ls -lht", "systemctl status ", "journalctl -u ", "tail -f ", "grep -rn ",
    ];
    const SNIPPET_STORE_KEY = "dbx_tp_snippets_v1";
    let customSnippets = loadCustomSnippets();

    function loadCustomSnippets() {
      try {
        const raw = localStorage.getItem(SNIPPET_STORE_KEY);
        const arr = raw ? JSON.parse(raw) : [];
        return Array.isArray(arr)
          ? arr.filter((s) => typeof s === "string" && s.length <= 200)
          : [];
      } catch (e) {
        return [];
      }
    }
    function persistCustomSnippets() {
      try {
        localStorage.setItem(SNIPPET_STORE_KEY, JSON.stringify(customSnippets));
      } catch (e) {
        // Sandbox may deny storage; custom snippets then live only this session.
      }
    }

    function renderSnippetsFor(sess) {
      const scroll = sess.el.querySelector(".snippet-scroll");
      scroll.innerHTML = "";
      const buildChip = (text, custom) => {
        const chip = document.createElement("button");
        chip.type = "button";
        chip.className = "snippet-chip" + (custom ? " custom" : "");
        const label = document.createElement("span");
        label.textContent = text;
        chip.title = text.slice(-1) === " "
          ? (t.snippetAddPh ? "点击插入，不自动执行" : "insert only")
          : "click to run";
        chip.appendChild(label);
        chip.addEventListener("click", () => sendSnippetTo(sess, text));
        if (custom) {
          const x = document.createElement("span");
          x.className = "chip-x";
          x.textContent = "×";
          x.addEventListener("click", (e) => {
            e.stopPropagation();
            customSnippets = customSnippets.filter((s) => s !== text);
            persistCustomSnippets();
            renderAllSnippets();
          });
          chip.appendChild(x);
        }
        scroll.appendChild(chip);
      };
      BUILTIN_SNIPPETS.forEach((s) => buildChip(s, false));
      customSnippets.forEach((s) => buildChip(s, true));
    }

    /// Re-render snippets across all open terminal sessions (e.g. after a
    /// custom snippet is added/removed).
    function renderAllSnippets() {
      terminalSessions.forEach((s) => { try { renderSnippetsFor(s); } catch (e) {} });
    }

    /// Send a snippet to a specific session's PTY.
    function sendSnippetTo(sess, text) {
      if (!sess || !sess.term) return;
      if (!sess.sessionId || sess.done) {
        try { sess.term.focus(); } catch (e) {}
        return;
      }
      try {
        const enc = new TextEncoder();
        window.dbxPlugin.sendBinary(`ssh/terminal/in/${sess.sessionId}`, enc.encode(text));
        if (text.slice(-1) !== " ") {
          window.dbxPlugin.sendBinary(`ssh/terminal/in/${sess.sessionId}`, enc.encode("\r"));
        }
        sess.term.focus();
      } catch (e) {}
    }

    async function runCardAction(act, r) {
      if (act === "ssh") {
        await openSshTerminal(r.name);
        return;
      }
      if (act === "forward") {
        openForwardOverlay(r.name);
        return;
      }
      if (act === "files") {
        openTransferOverlay(r.name);
        return;
      }
      if (act === "kube") {
        await kubeLogin(r.name);
        return;
      }
    }

    // ---------------------------------------------------------------------
    // Port forwarding (SSH local -L forwards)
    // ---------------------------------------------------------------------
    let fwdBusy = false;

    /// Populate the node datalist from currently loaded node resources.
    function populateNodeDatalist() {
      const dl = $("#fwdNodeList");
      const names = resources
        .filter((r) => r.kind === "node")
        .map((r) => r.name);
      dl.innerHTML = names.map(() => "<option>").join("");
      // option values assigned via DOM to avoid any HTML injection.
      [...dl.options].forEach((opt, i) => { opt.value = names[i]; });
    }

    function openForwardOverlay(prefillNode) {
      populateNodeDatalist();
      if (prefillNode) $("#fwdNode").value = prefillNode;
      openTab(TAB_FORWARD);
      $("#fwdMsg").textContent = "";
      refreshForwards();
    }

    async function createForward() {
      if (fwdBusy) return;
      const node = $("#fwdNode").value.trim();
      const localPort = parseInt($("#fwdLocalPort").value, 10);
      const targetHost = $("#fwdTargetHost").value.trim() || "127.0.0.1";
      const targetPort = parseInt($("#fwdTargetPort").value, 10);
      const msg = $("#fwdMsg");
      msg.classList.remove("ok");
      if (!node) { msg.textContent = t.sError + ": node"; return; }
      if (!(localPort >= 1 && localPort <= 65535)) {
        msg.textContent = t.sError + ": localPort"; return;
      }
      if (!(targetPort >= 1 && targetPort <= 65535)) {
        msg.textContent = t.sError + ": targetPort"; return;
      }

      fwdBusy = true;
      $("#fwdCreateBtn").disabled = true;
      msg.textContent = "";
      try {
        // Backend waits 2s to confirm the tunnel is actually up.
        const res = await callSidecar(
          "teleport/forwardStart",
          { connectionId, node, localPort, targetHost, targetPort },
          30000
        );
        msg.classList.add("ok");
        msg.textContent = t.fwdCreated.replace("{url}", res.local_url);
        await refreshForwards();
      } catch (e) {
        msg.textContent = String((e && e.message) || e);
      } finally {
        fwdBusy = false;
        $("#fwdCreateBtn").disabled = false;
      }
    }

    async function stopForward(id) {
      try {
        await callSidecar("teleport/forwardStop", { id }, 10000);
        // The monitor thread also emits forward/state; refresh right away.
        await refreshForwards();
      } catch (e) {
        const msg = $("#fwdMsg");
        msg.classList.remove("ok");
        msg.textContent = String((e && e.message) || e);
      }
    }

    async function refreshForwards() {
      let res;
      try {
        res = await callSidecar("teleport/forwardList", {}, 10000);
      } catch (e) {
        return;
      }
      renderFwdTable((res && res.forwards) || []);
    }

    function renderFwdTable(forwards) {
      const body = $("#fwdTableBody");
      if (!forwards.length) {
        body.innerHTML = `<tr><td colspan="4" style="color:var(--muted)">${t.fwdNone}</td></tr>`;
        return;
      }
      // Stable order: newest first is friendlier; keep by node name sort.
      forwards = [...forwards].sort((a, b) =>
        String(b.started_at).localeCompare(String(a.started_at)));

      body.innerHTML = forwards.map((f, i) => `
        <tr>
          <td class="mono" data-node="${i}"></td>
          <td class="mono" data-map="${i}"></td>
          <td><span class="fwd-state">${t.fwdRunning}</span></td>
          <td>
            <button type="button" data-copy="${i}">${t.fwdCopy}</button>
            <button type="button" data-stop="${i}">${t.fwdStop}</button>
          </td>
        </tr>`).join("");

      // All values through textContent — no HTML injection from tsh data.
      body.querySelectorAll("[data-node]").forEach((el) => {
        el.textContent = forwards[Number(el.dataset.node)].node;
      });
      body.querySelectorAll("[data-map]").forEach((el) => {
        const f = forwards[Number(el.dataset.map)];
        el.textContent = `${f.bind}:${f.local_port} → ${f.target_host}:${f.target_port}`;
      });
      body.querySelectorAll("[data-copy]").forEach((btn) => {
        btn.addEventListener("click", () => {
          const f = forwards[Number(btn.dataset.copy)];
          window.dbxPlugin.copy(f.local_url);
        });
      });
      body.querySelectorAll("[data-stop]").forEach((btn) => {
        btn.addEventListener("click", () => {
          stopForward(forwards[Number(btn.dataset.stop)].id);
        });
      });
    }

    // ---------------------------------------------------------------------
    // Kubernetes context injection
    // ---------------------------------------------------------------------
    async function kubeLogin(cluster) {
      const hint = $("#hint");
      try {
        const res = await callSidecar(
          "teleport/kubeLogin",
          { connectionId, cluster },
          100000
        );
        hint.hidden = false;
        hint.textContent = t.kubeDone
          .replace("{ctx}", res.context)
          .replace("{path}", res.kubeconfig);
      } catch (e) {
        hint.hidden = false;
        hint.textContent = `kube error: ${String((e && e.message) || e)}`;
      }
    }

    // ---------------------------------------------------------------------
    // Batch execution + liveness probe
    // ---------------------------------------------------------------------
    function updateBatchBar() {
      const n = selectedHosts.size;
      $("#batchBar").classList.toggle("open", n > 0);
      $("#batchSelected").textContent = t.selected.replace("{n}", String(n));
    }

    let probeProgress = 0;
    let retryProgress = 0;

    /// Handle a streamed `batch/item` event (probe liveness or exec row).
    function onBatchItem(params) {
      if (!params) return;
      if (params.kind === "probe" && params.batch_id === activeProbeId) {
        // The events only drive the progress label; the authoritative
        // online/slow result comes from the final probe response.
        if (params.phase === 2) {
          retryProgress++;
          $("#btnProbe").textContent = `${t.confirming.replace("{n}", String(retryProgress))}`;
        } else {
          probeProgress++;
          const nodeTotal = resources.filter((r) => r.kind === "node").length;
          $("#btnProbe").textContent = `${t.probing} ${probeProgress}/${nodeTotal}`;
        }
        return;
      }
      if (params.kind === "exec" && params.batch_id === activeBatchId) {
        batchRows.set(params.host, params);
        if ($("#batchOverlay").classList.contains("open")) renderBatchRows();
      }
    }

    async function runProbe() {
      if (busy || batchRunning) return;
      const btn = $("#btnProbe");
      btn.disabled = true;
      const probeId =
        "p" + Date.now().toString(36) + Math.random().toString(36).slice(2, 6);
      activeProbeId = probeId;
      probeProgress = 0;
      retryProgress = 0;
      const nodeTotal = resources.filter((r) => r.kind === "node").length;
      btn.textContent = `${t.probing} 0/${nodeTotal}`;
      try {
        // Two-phase probe (broad sweep + slow confirmation) can take several
        // minutes over satellite links; allow up to ten minutes.
        const res = await callSidecar(
          "teleport/probe",
          { connectionId, batchId: probeId },
          600000
        );
        if (res && res.online && typeof res.online === "object") {
          onlineMap = new Map(Object.entries(res.online));
        }
        if (res && res.slow && typeof res.slow === "object") {
          slowHosts = new Map(Object.entries(res.slow));
        } else {
          slowHosts = new Map();
        }
        probed = true;
        // Selected nodes that turned out offline can no longer be targets.
        [...selectedHosts].forEach((h) => {
          if (onlineMap.get(h) === false) selectedHosts.delete(h);
        });
        renderResources();
        updateBatchBar();
      } catch (e) {
        const hint = $("#hint");
        hint.hidden = false;
        hint.textContent = `probe error: ${String((e && e.message) || e)}`;
      } finally {
        btn.disabled = false;
        btn.textContent = t.probe;
        activeProbeId = null;
      }
    }

    // ── Fleet inspection (health heatmap wall) ────────────────────────────
    let inspData = new Map();       // hostname -> metrics item
    let inspRunning = false;
    let inspAutoTimer = null;
    let inspLastRun = null;         // Date
    let activeInspectId = null;
    let inspCollected = 0;
    let inspDetailHost = null;

    /// Classify one node for the heatmap: {cls: ok|warn|bad|off, sub}.
    function inspClassify(host) {
      if (onlineMap.get(host) === false) return { cls: "off", sub: t.inspOffline };
      const d = inspData.get(host);
      if (!d) return { cls: "off", sub: t.inspPending };
      if (d.status !== "ok") {
        return { cls: "bad", sub: d.status === "timeout" ? t.sTimeout : t.inspFailed };
      }
      const disk = d.disk_pct, mem = d.mem_pct, load = d.load1;
      if ((disk != null && disk >= 90) || (mem != null && mem >= 93)) {
        return {
          cls: "bad",
          sub: disk != null && disk >= 90 ? `${t.inspDisk} ${disk}%` : `${t.inspMem} ${mem}%`,
        };
      }
      if ((disk != null && disk >= 80) || (mem != null && mem >= 85) || (load != null && load >= 4)) {
        let why = `${t.inspLoad} ${load}`;
        if (disk != null && disk >= 80) why = `${t.inspDisk} ${disk}%`;
        else if (mem != null && mem >= 85) why = `${t.inspMem} ${mem}%`;
        return { cls: "warn", sub: why };
      }
      const parts = [];
      if (disk != null) parts.push(disk + "%");
      if (load != null) parts.push(Number(load).toFixed(1));
      return { cls: "ok", sub: parts.join(" / ") || t.inspOk };
    }

    function renderInspect() {
      const wall = $("#inspWall");
      const nodes = resources.filter((r) => r.kind === "node");
      const groups = new Map();
      for (const r of nodes) {
        const g = groupKey(r);
        if (!groups.has(g)) groups.set(g, []);
        groups.get(g).push(r);
      }
      const order = { bad: 0, warn: 1, off: 2, ok: 3 };
      const counts = { ok: 0, warn: 0, bad: 0, off: 0 };
      wall.innerHTML = "";
      for (const g of [...groups.keys()].sort((a, b) => a.localeCompare(b))) {
        const items = groups.get(g).map((r) => ({ r, c: inspClassify(r.name) }));
        items.sort((a, b) => order[a.c.cls] - order[b.c.cls] || a.r.name.localeCompare(b.r.name));
        for (const { c } of items) counts[c.cls]++;
        const badN = items.filter((x) => x.c.cls === "bad").length;

        const groupEl = document.createElement("div");
        groupEl.className = "insp-group";
        const head = document.createElement("div");
        head.className = "insp-group-head";
        const nm = document.createElement("span");
        nm.className = "insp-group-name";
        nm.textContent = g;
        const ct = document.createElement("span");
        ct.className = "insp-group-count";
        ct.textContent = String(items.length);
        if (badN) {
          const b = document.createElement("span");
          b.className = "gbad";
          b.textContent = ` · ${badN} ${t.inspBad}`;
          ct.appendChild(b);
        }
        head.append(nm, ct);
        const tiles = document.createElement("div");
        tiles.className = "insp-tiles";
        for (const { r, c } of items) {
          const tile = document.createElement("button");
          tile.type = "button";
          tile.className = "insp-tile " + c.cls;
          const tn = document.createElement("span");
          tn.className = "t-name";
          tn.textContent = r.name;
          tn.title = r.name;
          const ts = document.createElement("span");
          ts.className = "t-sub";
          ts.textContent = c.sub;
          tile.append(tn, ts);
          tile.addEventListener("click", () => showInspDetail(r.name));
          tiles.appendChild(tile);
        }
        groupEl.append(head, tiles);
        wall.appendChild(groupEl);
      }

      const chips = $("#inspChips");
      chips.innerHTML = "";
      const mk = (cls, label, n) => {
        const s = document.createElement("span");
        s.className = "insp-chip";
        const dot = document.createElement("i");
        dot.className = "insp-dot " + cls;
        const b = document.createElement("b");
        b.textContent = String(n);
        s.append(dot, document.createTextNode(label + " "), b);
        return s;
      };
      chips.append(
        mk("ok", t.inspOk, counts.ok),
        mk("warn", t.inspWarn, counts.warn),
        mk("bad", t.inspBad, counts.bad),
        mk("off", t.inspOff, counts.off)
      );

      const meta = $("#inspMeta");
      if (inspRunning) {
        meta.textContent = `${t.inspRunning} ${inspCollected}/${nodes.length}`;
      } else {
        meta.textContent = inspLastRun
          ? `${t.inspLast} ${inspLastRun.toLocaleTimeString()}`
          : t.inspNever;
      }
    }

    function onInspectItem(params) {
      if (!params || params.inspect_id !== activeInspectId) return;
      inspData.set(params.host, params);
      inspCollected++;
      if ($("#inspectOverlay").classList.contains("open")) renderInspect();
      // Keep an open detail panel in sync as fresh rows arrive.
      if (inspDetailHost === params.host && !$("#inspDetail").hidden) {
        showInspDetail(inspDetailHost);
      }
    }

    function showInspDetail(host) {
      inspDetailHost = host;
      const det = $("#inspDetail");
      det.hidden = false;
      $("#inspDetailHost").textContent = host;
      const grid = $("#inspDetailGrid");
      grid.innerHTML = "";
      const errEl = $("#inspDetailErr");
      const mkMetric = (label, value, cls) => {
        const m = document.createElement("div");
        m.className = "insp-metric" + (cls ? " " + cls : "");
        const l = document.createElement("div");
        l.className = "m-label";
        l.textContent = label;
        const v = document.createElement("div");
        v.className = "m-value";
        v.textContent = value;
        m.append(l, v);
        return m;
      };
      const statusText = { ok: t.sOk, fail: t.sFail, timeout: t.sTimeout, error: t.sError };
      const d = inspData.get(host);
      if (!d) {
        grid.append(mkMetric(t.bColStatus, inspClassify(host).sub));
        errEl.hidden = true;
        return;
      }
      const diskCls = d.disk_pct >= 90 ? "bad" : d.disk_pct >= 80 ? "warn" : "";
      const memCls = d.mem_pct >= 93 ? "bad" : d.mem_pct >= 85 ? "warn" : "";
      const loadCls = d.load1 >= 4 ? "warn" : "";
      grid.append(
        mkMetric(t.bColStatus, statusText[d.status] || d.status),
        mkMetric(t.inspDisk, d.disk_pct != null ? `${d.disk_pct}% (${d.disk_avail})` : "—", diskCls),
        mkMetric(t.inspMem, d.mem_pct != null ? `${d.mem_pct}%` : "—", memCls),
        mkMetric(t.inspLoad, d.load1 != null ? String(d.load1) : "—", loadCls),
        mkMetric(t.inspDocker, d.containers != null ? String(d.containers) : t.inspNoDocker),
        mkMetric(t.inspDuration, d.duration_ms != null ? `${d.duration_ms}ms` : "—")
      );
      if (d.stderr) {
        errEl.hidden = false;
        errEl.textContent = d.stderr;
      } else {
        errEl.hidden = true;
      }
    }

    async function runInspect() {
      if (inspRunning) return;
      const nodes = resources.filter((r) => r.kind === "node");
      if (!nodes.length) return;
      inspRunning = true;
      inspCollected = 0;
      const btn = $("#inspRefreshBtn");
      btn.disabled = true;
      renderInspect();
      const inspectId = "i" + Date.now().toString(36) + Math.random().toString(36).slice(2, 7);
      activeInspectId = inspectId;
      try {
        // Skip known-offline nodes — no point burning satellite airtime on
        // them; they stay grey on the wall.
        const hosts = nodes.map((r) => r.name).filter((h) => onlineMap.get(h) !== false);
        await callSidecar(
          "teleport/inspect",
          { connectionId, hosts, concurrency: 20, timeoutSecs: 60, batchId: inspectId },
          600000
        );
        inspLastRun = new Date();
      } catch (e) {
        const hint = $("#hint");
        hint.hidden = false;
        hint.textContent = `inspect error: ${String((e && e.message) || e)}`;
      } finally {
        inspRunning = false;
        activeInspectId = null;
        btn.disabled = false;
        renderInspect();
      }
    }

    function openInspectOverlay() {
      openTab(TAB_INSPECT);
      renderInspect();
      if (!inspData.size && !inspRunning) runInspect();
    }

    function setInspectAuto(on) {
      if (inspAutoTimer) {
        clearInterval(inspAutoTimer);
        inspAutoTimer = null;
      }
      if (on) {
        inspAutoTimer = setInterval(() => {
          if (!inspRunning && $("#inspectOverlay").classList.contains("open")) runInspect();
        }, 600000);
      }
    }

    function setBatchRunning(v) {
      batchRunning = v;
      $("#batchRunBtn").disabled = v;
    }

    async function runBatch() {
      if (batchRunning) return;
      const command = $("#batchCommand").value.trim();
      if (!command) {
        $("#batchCommand").focus();
        return;
      }
      const hosts = [...selectedHosts];
      if (!hosts.length) return;
      const concurrency = Math.min(
        50,
        Math.max(1, parseInt($("#batchConcurrency").value, 10) || 10)
      );
      const timeoutSecs = Math.min(
        600,
        Math.max(1, parseInt($("#batchTimeout").value, 10) || 60)
      );
      const batchId =
        "c" + Date.now().toString(36) + Math.random().toString(36).slice(2, 7);
      activeBatchId = batchId;
      batchRows = new Map();
      batchExpanded = new Set();
      hosts.forEach((h) =>
        batchRows.set(h, {
          host: h,
          status: "pending",
          duration_ms: null,
          stdout: "",
          stderr: "",
        })
      );

      openBatchOverlay(command);
      renderBatchRows();
      setBatchRunning(true);
      let summary = null;
      try {
        summary = await callSidecar(
          "teleport/batchExec",
          { connectionId, hosts, command, concurrency, timeoutSecs, batchId },
          Math.max(180000, timeoutSecs * 1000 + 90000)
        );
      } catch (e) {
        batchRows.forEach((r) => {
          if (r.status === "pending") {
            r.status = "error";
            r.stderr = String((e && e.message) || e);
          }
        });
      }
      if (summary && Array.isArray(summary.results)) {
        batchRows = new Map(summary.results.map((r) => [r.host, r]));
      }
      renderBatchRows();
      updateBatchSummary(summary);
      setBatchRunning(false);
    }

    let lastBatchCommand = "";
    let lastBatchSummary = null;

    function openBatchOverlay(command) {
      lastBatchCommand = command;
      lastBatchSummary = null;
      openTab(TAB_BATCH, `${t.batch} › ${command}`.slice(0, 40));
      $("#batchOverlayTitle").textContent = `${t.batch} › ${command}`;
      $("#batchSummary").textContent = "";
      // Start on the table view; report view stays hidden until requested.
      $("#batchResultBody").hidden = false;
      $("#batchReportView").hidden = true;
      $("#batchReportBtn").textContent = t.report;
    }

    function renderBatchRows() {
      const body = $("#batchResultBody");
      const hosts = [...batchRows.keys()];
      const statusLabel = {
        pending: t.sPending,
        ok: t.sOk,
        fail: t.sFail,
        timeout: t.sTimeout,
        error: t.sError,
      };
      const rowsHtml = hosts
        .map((h, i) => {
          const r = batchRows.get(h);
          const expanded = batchExpanded.has(h);
          const dur = r.duration_ms != null ? `${r.duration_ms}ms` : "—";
          const hasOut = !!(r.stdout || r.stderr);
          const outRow =
            expanded && hasOut
              ? `<tr><td colspan="3" class="bout-cell">
                  ${r.stdout ? `<pre class="bout" data-bout-out="${i}"></pre>` : ""}
                  ${r.stderr ? `<pre class="bout err" data-bout-err="${i}"></pre>` : ""}
                </td></tr>`
              : "";
          return `<tr class="row-main" data-row-idx="${i}">
            <td class="bhost" data-row-host="${i}"></td>
            <td><span class="bstatus ${r.status}">${statusLabel[r.status] || r.status}</span></td>
            <td class="bdur">${dur}</td>
          </tr>${outRow}`;
        })
        .join("");
      body.innerHTML = `<table class="bresult-table">
        <thead><tr><th>${t.bColHost}</th><th>${t.bColStatus}</th><th>${t.bColDuration}</th></tr></thead>
        <tbody>${rowsHtml}</tbody>
      </table>`;

      // All host/output text goes through textContent — no HTML injection.
      body.querySelectorAll("[data-row-host]").forEach((el) => {
        el.textContent = hosts[Number(el.dataset.rowHost)];
      });
      body.querySelectorAll("[data-bout-out]").forEach((el) => {
        el.textContent = batchRows.get(hosts[Number(el.dataset.boutOut)]).stdout;
      });
      body.querySelectorAll("[data-bout-err]").forEach((el) => {
        el.textContent = batchRows.get(hosts[Number(el.dataset.boutErr)]).stderr;
      });
      body.querySelectorAll("tr.row-main").forEach((tr) => {
        tr.addEventListener("click", () => {
          const h = hosts[Number(tr.dataset.rowIdx)];
          const r = batchRows.get(h);
          if (!(r.stdout || r.stderr)) return;
          if (batchExpanded.has(h)) batchExpanded.delete(h);
          else batchExpanded.add(h);
          renderBatchRows();
        });
      });
    }

    function updateBatchSummary(summary) {
      lastBatchSummary = summary;
      if (!summary) return;
      $("#batchSummary").textContent =
        t.bDone
          .replace("{ok}", summary.ok)
          .replace("{total}", summary.total)
          .replace("{fail}", summary.failed) +
        ` · ${summary.duration_ms}ms`;
    }

    // -----------------------------------------------------------------------
    // Consolidated per-node report (one text block, printable / copyable)
    // -----------------------------------------------------------------------
    function buildBatchReport() {
      const hosts = [...batchRows.keys()].sort();
      const rows = hosts.map((h) => batchRows.get(h));

      // Duration statistics over finished nodes.
      const durs = rows
        .map((r) => (typeof r.duration_ms === "number" ? r.duration_ms : null))
        .filter((d) => d !== null);
      const sum = durs.reduce((a, b) => a + b, 0);
      const avg = durs.length ? Math.round(sum / durs.length) : 0;
      const min = durs.length ? Math.min(...durs) : 0;
      const max = durs.length ? Math.max(...durs) : 0;
      const totalMs = lastBatchSummary && typeof lastBatchSummary.duration_ms === "number"
        ? lastBatchSummary.duration_ms
        : sum;

      const okCount = rows.filter((r) => r.status === "ok").length;
      const failCount = rows.length - okCount;

      const now = new Date();
      const pad = (n) => String(n).padStart(2, "0");
      const timeStr =
        `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())} ` +
        `${pad(now.getHours())}:${pad(now.getMinutes())}:${pad(now.getSeconds())}`;

      const statusTag = {
        ok: "OK",
        fail: "FAIL",
        timeout: "TIMEOUT",
        error: "ERROR",
        pending: "PENDING",
      };

      const lines = [];
      lines.push(`===== ${t.rptTitle} =====`);
      lines.push(`${t.rptCmd}: ${lastBatchCommand}`);
      lines.push(`${t.rptTime}: ${timeStr}`);
      lines.push(
        t.rptStat
          .replace("{total}", rows.length)
          .replace("{ok}", okCount)
          .replace("{fail}", failCount)
      );
      lines.push(
        t.rptDur
          .replace("{totalMs}", totalMs)
          .replace("{avgMs}", avg)
          .replace("{minMs}", min)
          .replace("{maxMs}", max)
      );
      lines.push("");
      lines.push(`----- ${t.rptDetail} (${rows.length}) -----`);

      rows.forEach((r, i) => {
        const tag = statusTag[r.status] || r.status.toUpperCase();
        const dur = typeof r.duration_ms === "number" ? `${r.duration_ms}ms` : "-";
        lines.push(`[${tag}] ${r.host}  (${dur})`);
        const stdout = (r.stdout || "").replace(/\s+$/, "");
        const stderr = (r.stderr || "").replace(/\s+$/, "");
        if (stdout) {
          lines.push("  stdout:");
          stdout.split("\n").forEach((l) => lines.push("    " + l));
        }
        if (stderr) {
          lines.push("  stderr:");
          stderr.split("\n").forEach((l) => lines.push("    " + l));
        }
        if (!stdout && !stderr) lines.push("  " + t.rptNoOutput);
        if (i < rows.length - 1) lines.push("");
      });

      return lines.join("\n");
    }

    /// Toggle between the table view and the full per-node report view.
    function toggleBatchReport() {
      const view = $("#batchReportView");
      const table = $("#batchResultBody");
      const btn = $("#batchReportBtn");
      if (view.hidden) {
        $("#batchReportPre").textContent = buildBatchReport();
        view.hidden = false;
        table.hidden = true;
        btn.textContent = t.backToTable;
      } else {
        view.hidden = true;
        table.hidden = false;
        btn.textContent = t.report;
      }
    }

    /// Copy the full report to the clipboard via the host bridge.
    function copyBatchReport() {
      try {
        window.dbxPlugin.copy(buildBatchReport());
      } catch (e) {}
    }

    // -----------------------------------------------------------------------
    // File transfer (upload / download via tsh scp)
    // -----------------------------------------------------------------------
    let xferContextNode = null;
    const xferRows = new Map(); // node -> {status, duration_ms, stderr}
    let xferBusy = false;

    function openTransferOverlay(node) {
      xferContextNode = node || null;
      $("#xferDownNode").value = node || "";
      $("#xferUpLocal").value = "";
      $("#xferUpRemote").value = "/root";
      $("#xferUpRecursive").checked = false;
      $("#xferUpAll").checked = false;
      $("#xferDownRemote").value = "";
      $("#xferDownLocal").value = "";
      $("#xferDownRecursive").checked = false;
      $("#xferUpMsg").textContent = "";
      $("#xferDownMsg").textContent = "";
      xferRows.clear();
      renderXferRows();
      updatePushAllLabel();
      switchXferTab("up");
      openTab(TAB_TRANSFER);
    }

    function updatePushAllLabel() {
      const n = selectedHosts.size;
      $("#xferUpAllLabel").textContent =
        n > 0 ? t.xferPushAll.replace("{n}", n) : t.xferNodeSingle;
    }

    function switchXferTab(mode) {
      const up = mode === "up";
      $("#xferTabUp").classList.toggle("active", up);
      $("#xferTabDown").classList.toggle("active", !up);
      $("#xferPaneUp").hidden = !up;
      $("#xferPaneDown").hidden = up;
    }

    /// Explorer's "Copy as path" wraps the path in quotes; strip them.
    function stripQuotes(p) {
      let s = p.trim();
      if (s.length >= 2 && s.startsWith('"') && s.endsWith('"')) {
        s = s.slice(1, -1);
      }
      return s;
    }

    function onTransferItem(p) {
      xferRows.set(p.node, {
        status: p.status,
        duration_ms: p.duration_ms,
        stderr: p.stderr || "",
      });
      renderXferRows();
    }

    function renderXferRows() {
      const body = $("#xferUpBody");
      const hosts = [...xferRows.keys()];
      const stateLabel = {
        pending: t.sPending, ok: t.sOk, fail: t.sFail,
        timeout: t.sTimeout, error: t.sFail,
      };
      body.innerHTML = hosts.map((h, i) => `
        <tr>
          <td class="mono" data-node="${i}"></td>
          <td><span class="fwd-state" data-state="${i}"></span></td>
          <td class="mono" data-dur="${i}"></td>
          <td class="xfer-fail" data-err="${i}"></td>
        </tr>`).join("");

      body.querySelectorAll("[data-node]").forEach((el) => {
        el.textContent = hosts[Number(el.dataset.node)];
      });
      body.querySelectorAll("[data-state]").forEach((el) => {
        const r = xferRows.get(hosts[Number(el.dataset.state)]);
        el.textContent = stateLabel[r.status] || r.status;
        el.style.color =
          r.status === "ok" ? "var(--brand)"
          : r.status === "timeout" ? "#b9770e"
          : r.status === "pending" ? "var(--muted)"
          : "var(--danger)";
      });
      body.querySelectorAll("[data-dur]").forEach((el) => {
        const r = xferRows.get(hosts[Number(el.dataset.dur)]);
        el.textContent =
          typeof r.duration_ms === "number" && r.status !== "pending"
            ? r.duration_ms + "ms" : "";
      });
      body.querySelectorAll("[data-err]").forEach((el) => {
        const r = xferRows.get(hosts[Number(el.dataset.err)]);
        el.textContent = r.status === "ok" ? "" : (r.stderr || "");
      });
    }

    async function startUpload() {
      if (xferBusy) return;
      const localPath = stripQuotes($("#xferUpLocal").value);
      const remotePath = $("#xferUpRemote").value.trim();
      const recursive = $("#xferUpRecursive").checked;
      const pushAll = $("#xferUpAll").checked && selectedHosts.size > 0;
      const nodes = pushAll
        ? [...selectedHosts]
        : (xferContextNode ? [xferContextNode] : []);

      const msg = $("#xferUpMsg");
      msg.classList.remove("ok");
      if (!localPath) { msg.textContent = t.sError + ": local"; return; }
      if (!remotePath) { msg.textContent = t.sError + ": remote"; return; }
      if (!nodes.length) { msg.textContent = t.sError + ": node"; return; }

      xferBusy = true;
      $("#xferUpBtn").disabled = true;
      xferRows.clear();
      nodes.forEach((n) =>
        xferRows.set(n, { status: "pending", duration_ms: null, stderr: "" }));
      renderXferRows();
      msg.textContent = "";

      try {
        const summary = await callSidecar(
          "teleport/transferUpload",
          {
            connectionId, localPath, remotePath, nodes, recursive,
            concurrency: 10, timeoutSecs: 300,
          },
          600000
        );
        // Final state comes from summary results (events may have already
        // painted everything; this guarantees the last row is correct).
        (summary.results || []).forEach((r) => onTransferItem(r));
        msg.classList.add("ok");
        msg.textContent = t.xferUpDone
          .replace("{ok}", summary.ok)
          .replace("{total}", summary.total);
      } catch (e) {
        msg.textContent = String((e && e.message) || e);
      } finally {
        xferBusy = false;
        $("#xferUpBtn").disabled = false;
      }
    }

    async function startDownload() {
      if (xferBusy) return;
      const node = $("#xferDownNode").value.trim();
      const remotePath = $("#xferDownRemote").value.trim();
      const localPath = stripQuotes($("#xferDownLocal").value);
      const recursive = $("#xferDownRecursive").checked;
      const msg = $("#xferDownMsg");
      msg.classList.remove("ok");
      if (!node || !remotePath || !localPath) {
        msg.textContent = t.sError;
        return;
      }

      xferBusy = true;
      $("#xferDownBtn").disabled = true;
      msg.textContent = t.xferRunning;
      try {
        await callSidecar(
          "teleport/transferDownload",
          { connectionId, node, remotePath, localPath, recursive, timeoutSecs: 300 },
          600000
        );
        msg.classList.add("ok");
        msg.textContent = t.xferDownDone.replace("{path}", localPath);
      } catch (e) {
        msg.textContent = String((e && e.message) || e);
      } finally {
        xferBusy = false;
        $("#xferDownBtn").disabled = false;
      }
    }

    $("#batchReportBtn").addEventListener("click", toggleBatchReport);
    $("#batchCopyReportBtn").addEventListener("click", copyBatchReport);

    // Transfer overlay.
    $("#xferTabUp").addEventListener("click", () => switchXferTab("up"));
    $("#xferTabDown").addEventListener("click", () => switchXferTab("down"));
    $("#xferUpBtn").addEventListener("click", startUpload);
    $("#xferDownBtn").addEventListener("click", startDownload);
    $("#transferOverlayClose").addEventListener("click", () => {
      if (xferBusy) return;
      const tab = openTabs.find((t) => t.type === TAB_TRANSFER);
      if (tab) closeTab(tab.id);
    });

    // Terminal controls are now wired per-session inside createTerminalSession().

    $("#batchToggleForm").addEventListener("click", () => {
      $("#batchForm").classList.toggle("open");
    });
    $("#batchClearBtn").addEventListener("click", () => {
      selectedHosts.clear();
      renderResources();
      updateBatchBar();
    });
    $("#batchRunBtn").addEventListener("click", runBatch);
    $("#batchOverlayClose").addEventListener("click", () => {
      // Keep the overlay while a batch is in flight so rows keep streaming in.
      if (batchRunning) return;
      const tab = openTabs.find((t) => t.type === TAB_BATCH);
      if (tab) closeTab(tab.id);
      activeBatchId = null;
    });
    $("#btnProbe").addEventListener("click", runProbe);
    $("#btnInspect").addEventListener("click", openInspectOverlay);
    $("#inspRefreshBtn").addEventListener("click", runInspect);
    $("#inspectOverlayClose").addEventListener("click", () => {
      $("#inspDetail").hidden = true;
      setInspectAuto(false);
      const tab = openTabs.find((t) => t.type === TAB_INSPECT);
      if (tab) closeTab(tab.id);
    });
    $("#inspAuto").addEventListener("change", (e) => setInspectAuto(e.target.checked));
    $("#inspDetailClose").addEventListener("click", () => {
      $("#inspDetail").hidden = true;
      inspDetailHost = null;
    });
    $("#inspDetailSsh").addEventListener("click", () => {
      if (inspDetailHost) openSshTerminal(inspDetailHost);
    });

    $("#fwdCreateBtn").addEventListener("click", createForward);
    $("#forwardOverlayClose").addEventListener("click", () => {
      if (fwdBusy) return;
      const tab = openTabs.find((t) => t.type === TAB_FORWARD);
      if (tab) closeTab(tab.id);
    });
    // Enter anywhere in the create form submits it.
    ["fwdNode", "fwdLocalPort", "fwdTargetHost", "fwdTargetPort"].forEach((id) => {
      $("#" + id).addEventListener("keydown", (e) => {
        if (e.key === "Enter") createForward();
      });
    });

    $("#btnLogout").addEventListener("click", async () => {
      $("#btnLogout").disabled = true;
      try {
        await callSidecar("connection/disconnect", { connectionId }, 15000);
      } finally {
        // A logout invalidates everything derived from the old session:
        // probe results, inspection metrics, and the batch selection all
        // belong to credentials that no longer exist.
        [...openTabs].forEach((tab) => { if (tab.type !== TAB_HOME) closeTab(tab.id); });
        selectedHosts.clear();
        onlineMap.clear();
        slowHosts.clear();
        inspData.clear();
        inspLastRun = null;
        probed = false;
        updateBatchBar();
        await refresh();
        $("#btnLogout").disabled = false;
      }
    });

    // Diagnostics bundle: tsh version/status + masked sidecar log, written to
    // %TEMP%; the path is copied to the clipboard for easy issue reporting.
    $("#btnDiag").addEventListener("click", async () => {
      const btn = $("#btnDiag");
      btn.disabled = true;
      try {
        const r = await callSidecar("teleport/diagBundle", { connectionId }, 60000);
        const path = (r && r.path) || "";
        if (path) { try { await window.dbxPlugin.copy(path); } catch (e) {} }
        const hint = $("#hint");
        hint.hidden = false;
        hint.textContent = t.diagSaved + path;
      } catch (e) {
        const hint = $("#hint");
        hint.hidden = false;
        hint.textContent = `diag error: ${String((e && e.message) || e)}`;
      } finally {
        btn.disabled = false;
      }
    });

    $("#filters").addEventListener("click", (e) => {
      const btn = e.target.closest("button[data-filter]");
      if (!btn) return;
      filter = btn.dataset.filter;
      document.querySelectorAll("#filters button").forEach((b) => b.classList.toggle("active", b === btn));
      renderResources();
    });

    $("#searchInput").addEventListener("input", (e) => {
      search = e.target.value;
      renderResources();
    });

    window.dbxPlugin.ready.then((ctx) => {
      try {
        // 保险：加载时关闭所有功能标签页，回到首页（资源列表）
        [...openTabs].forEach((tab) => closeTab(tab.id));
        activateTab(ensureHomeTab().id);
        const locale = (window.dbxPlugin.locale || "zh-CN").toLowerCase();
        t = I18N[locale] || I18N["zh-cn"];
        connectionId = (ctx && ctx.connectionId) || window.dbxPlugin.context?.connectionId || null;
        applyI18n();

        // Streamed batch items: liveness probes and per-node exec results.
        window.dbxPlugin.onEvent((ev) => {
          if (ev && ev.method === "batch/item") onBatchItem(ev.params || {});
          if (ev && ev.method === "forward/state") {
            // A tunnel dropped or was stopped; update the table.
            refreshForwards();
          }
          if (ev && ev.method === "transfer/item") {
            onTransferItem(ev.params || {});
          }
          if (ev && ev.method === "inspect/item") {
            onInspectItem(ev.params || {});
          }
        });

        // Route binary terminal output frames to the active (or pending) session.
        // DBX delivers plugin binary output on the "binary" message, which the
        // bridge exposes as onBinary with data already as a Uint8Array (NOT via
        // onEvent / decodeBase64 — those only carry JSON events).
        window.dbxPlugin.onBinary((ev) => {
          if (ev && isTerminalChannel(ev.channel)) {
            const sid = ev.channel.slice("ssh/terminal/out/".length);
            const bytes = ev.data instanceof Uint8Array ? ev.data : (ev.data ? new Uint8Array(ev.data) : null);
            if (!bytes || bytes.length < 9) return;
            const stream = bytes[0];
            let seq;
            try {
              seq = Number(new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getBigUint64(1, false));
            } catch (e) { return; }
            onTerminalFrame(sid, stream, seq, bytes.slice(9));
          }
        });

        refresh();
      } catch (e) {
        const hint = $("#hint");
        hint.hidden = false;
        hint.textContent = `init error: ${String((e && e.message) || e)}`;
      }
    });
