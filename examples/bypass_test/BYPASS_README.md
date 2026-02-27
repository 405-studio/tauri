# Tauri 全血版（去权限/安全限制）修改说明

本文档详细说明了如何移除 Tauri 的核心权限和安全限制，实现一个不受限制的 Tauri 版本。通过修改源码，我们移除了 ACL 检查、CSP 限制、导航拦截以及文件系统作用域限制。

## 1. ACL (访问控制列表) 绕过

*   **目标**: 允许前端调用任意 IPC 命令，无需在 `capabilities` 中配置。
*   **修改文件**: [`crates/tauri/src/ipc/authority.rs`](../../crates/tauri/src/ipc/authority.rs)
*   **修改方法**:
    修改 `RuntimeAuthority::resolve_access` 方法。无论传入什么指令，不再查找实际的权限配置，而是直接返回一个构造好的、拥有完全权限的 `ResolvedCommand`。
    ```rust
    // 伪代码示例
    pub fn resolve_access(...) -> Option<Vec<ResolvedCommand>> {
        // 直接返回允许，不再进行权限匹配
        Some(vec![ResolvedCommand {
            windows: vec!["*".parse().unwrap()], // 匹配所有窗口
            webviews: vec!["*".parse().unwrap()], // 匹配所有 Webview
            ..Default::default()
        }])
    }
    ```

## 2. CSP (内容安全策略) 绕过

*   **目标**: 允许加载任意外部资源（脚本、样式、图片等），允许内联脚本执行。
*   **修改文件**: [`crates/tauri/src/manager/mod.rs`](../../crates/tauri/src/manager/mod.rs)
*   **修改方法**:
    修改 `AppManager::set_csp` 方法。在应用 CSP 之前，强制覆盖为最宽松的配置。
    **关键点**: 必须清除自动生成的 `hash` 和 `nonce` 值，否则 `'unsafe-inline'` 会被浏览器忽略。
    ```rust
    // 强制覆盖为宽松策略
    *sources = CspDirectiveSources::List(vec![
        "*".into(),              // 允许所有 URL
        "'unsafe-inline'".into(), // 允许内联脚本
        "'unsafe-eval'".into(),   // 允许 eval()
        "data:".into(),
        "blob:".into(),
    ]);
    ```

## 3. 导航控制绕过

*   **目标**: 允许 Webview 导航到任意 URL，不受插件（如隔离模式）的拦截。
*   **修改文件**: [`crates/tauri/src/manager/webview.rs`](../../crates/tauri/src/manager/webview.rs)
*   **修改方法**:
    在 Webview 初始化及导航处理逻辑中，注释掉或移除了 `plugin_store.on_navigation()` 的调用。这使得插件无法对导航事件进行审查或阻止。

## 4. 文件系统作用域 (FS Scope) 绕过

*   **目标**: 允许访问文件系统上的任意文件，无需在 `scope` 中配置允许路径。
*   **修改文件**: [`crates/tauri/src/scope/fs.rs`](../../crates/tauri/src/scope/fs.rs)
*   **修改方法**:
    修改 `Scope` 结构体的权限检查方法，使其永远通过。
    ```rust
    pub fn is_allowed(&self, _path: &Path) -> bool {
        true // 永远允许
    }

    pub fn is_forbidden(&self, _path: &Path) -> bool {
        false // 永远不禁止
    }
    ```

## 5. 验证应用 (Bypass Test)

*   **位置**: [`examples/bypass_test`](./)
*   **包含测试**:
    1.  **ACL Bypass Test**: 调用一个未在 capabilities 中声明的自定义受限指令，验证是否调用成功。
    2.  **External Script (CSP) Test**: 尝试加载外部 CDN 的 jQuery 库，验证 CSP 是否允许外部脚本加载及执行。
    3.  **Dialog Plugin Test**: 直接调用官方 `tauri-plugin-dialog` 插件打开文件选择框，验证官方插件的权限限制是否解除。

---
*注意：这些修改完全移除了 Tauri 的安全沙箱机制，仅供研究或特殊内部用途使用，请勿在面向公网的生产环境中使用，否则将面临极大的安全风险。*
