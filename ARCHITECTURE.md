# Weixin Architecture & Refactor Plan

## Goals
- Preserve current behavior and UI.
- Introduce clearer layering and boundaries for future growth.
- Reduce duplication and centralize cross-cutting concerns.

## Current Status (Phase 1 completed, behavior preserved)
- Layers in place:
  - src/app/{state.rs,view_builder.rs,event_handlers.rs,events.rs}
  - src/infra/{mod.rs,memory_repos.rs,sample_data.rs}
  - src/ui/{constants.rs,theme.rs,widgets/{mod.rs,window_controls.rs, ...}}
  - src/components/{chat_area.rs,session_list.rs,toolbar.rs,settings_window.rs}（后续将重组为按功能分包）
- Removed/Consolidated:
  - Deleted src/core 与 src/domain；统一事件为 app::events::AppEvent。
  - Moved src/data/sample_data.rs 到 src/infra/sample_data.rs，删除 src/data。
  - 移除 src/theme.rs（直接使用 ui::theme）。
  - 移除未用的 ui/prelude.rs、ui/widgets/popover.rs 与 ui/widgets/toolbar/nav_button.rs 占位；group_avatar 移至 ui/widgets。
- App wiring:
  - WeixinApp 直接使用 infra::memory_repos（不再通过 domain traits）。
  - 事件订阅统一由 app/event_handlers.rs 处理 AppEvent。
- UI 去重：
  - ChatArea: 抽出 `icon_button`/`narrow_icon_button`。
  - SettingsWindow: 抽出 `render_static_select_item`（主题/语言/字号）。
  - 提取 `ui::widgets::window_controls`、`search_area`、`chat_header_actions`、`session_row` 等。
  - 标准化 toolbar 触发尺寸 `toolbar_trigger_size`；常量统一到 `ui::constants`。

## Target Layout
- src/app/
  - state.rs
  - view_builder.rs
  - event_handlers.rs
  - events.rs (AppEvent)
- src/infra/
  - memory_repos.rs
  - sample_data.rs
- src/ui/
  - constants.rs (UI dimensions and tokens)
  - theme.rs (WeixinThemeColors / helpers)
  - widgets/
    - window_controls.rs, search_area.rs, chat_header_actions.rs
    - session_row.rs, badge_avatar.rs, icon_buttons.rs
    - chat_toolbar.rs, message_bubble.rs, message_list.rs
    - toolbar/hover_menu_item.rs
    - （deferred）toolbar triggers / standardized popover wrapper
- src/components/
  - chat/{mod.rs,view.rs}（原 chat_area.rs）
  - sessions/{mod.rs,view.rs}（原 session_list.rs）
  - settings/{mod.rs,window.rs}（原 settings_window.rs）
  - sidebar/{mod.rs,view.rs}（原 toolbar.rs）
  - mod.rs（re-export ChatView, SessionsView, SettingsWindow, SidebarView）

## Data Boundary
- 无 domain 层；直接使用 infra::memory_repos，数据由 app/state 读写并下发到组件。
- UI（components/ui::widgets）不可直接依赖 infra；只能通过 app/state 提供的数据更新 UI。
- infra::sample_data 仅作为开发示例数据来源，未来可替换为持久化实现。

## Events Boundary
- 仅保留一套事件：app::events::AppEvent（SessionSelected、ToolbarClicked）。
- UI 发出 AppEvent，app/event_handlers 统一订阅并调度（例如切换会话、打开设置窗口等）。

## Refactor Steps
- Phase A（done）
  1) 移除 domain & 统一事件为 app::events::AppEvent。
  2) 数据改为直接使用 infra::memory_repos；删除 src/theme.rs，改为 ui::theme。
  3) 提取/整合 UI widgets 与常量，删除未用与占位组件。
- Phase B（components 目录重组，行为保持）
  - 新建 components/{chat,sessions,settings,sidebar} 分包；分别迁移现有 *rs 文件为 {view.rs / window.rs}；mod.rs 做对外导出。
  - components/mod.rs 仅导出四个容器视图；内部引用 ui::widgets 构建 UI。
- Phase C（Non-toolbar UI improvements，行为保持）
  - ChatArea：时间分组/日期分隔统一；message_input 容器抽出；输入区粘底/自适应高度。
  - MessageList：大列表渐进渲染与粘底滚动稳定性；保持选择/光标。
  - SessionList：行高/间距统一，头像与徽章对齐；文本溢出处理与时间戳右对齐。
  - Search：防抖与命中高亮；空态与清除按钮。
  - A11y：tab 顺序/焦点样式；快捷键。
- Deferred: Toolbar extraction
  - 暂缓 toolbar 更换触发与标准 Popover 容器，保持现状，后续一并替换。

## Deprecations / Removals
- Removed: domain/ 整体；统一事件为 app::events::AppEvent。
- Removed: src/theme.rs（直接使用 ui::theme）。
- Removed: ChatArea 内部 icon_button/narrow_icon_button（由 ui::widgets::icon_buttons 替代）。
- Replaced: ChatArea::render_message（由 ui::widgets::message_bubble/message_list 替代）。
- Removed: ToolBar::render_hover_menu_item（已用 widgets::toolbar::hover_menu_item 全部替换）。
- Deferred: Toolbar 导航按钮与标准 Popover 容器（已移除相关占位组件）。
- Moved: sample_data 到 infra；group_avatar 到 ui/widgets。

## UI Architecture & Conventions
- Responsibilities
  - app/: composition root（布局与依赖注入），不包含复杂 UI 片段实现。
  - components/: 具体视图（ChatArea/SessionList/Toolbar 等），通过组合 ui::widgets 构建界面。
  - ui/widgets/: 纯 UI 组件，无业务状态，只接收必要的 props（theme/weixin_colors/数据切片）。
  - ui/theme: 主题与颜色，仅暴露通过 Theme::weixin_colors(cx) 获取的颜色。
  - ui/constants: 尺寸/间距/字号等 UI 常量。
- Dependency rules
  - ui/widgets 只能依赖 ui::{theme,constants} 与 gpui(-component)，不得依赖 infra。
  - components 不得直接依赖 infra；数据通过 app/state 注入（由 app 调用 infra 实现）。
- Styling
  - 尽量使用 ui::constants；避免硬编码 magic numbers。
  - 交互/hover 效果保持与 theme/colors 一致，必要时封装为小部件（如 window_controls、icon_buttons、toolbar/*）。

### UI Constants (current tokens)
- toolbar_trigger_size: 41px（phone/menu 触发按钮）
- window_button_width: 45px；title_bar_height: 67px；toolbar_width: 67px；settings_title_height: 48px
- session_list_min_width: 200px；session_list_max_width: 400px
- icon sizes: 20px/21px（内容/工具栏图标）
- chat_input_default_height: 200px；chat_input_min_height: 120px；chat_input_max_height: 420px
- avatar_large: 46px；avatar_small: 35px
- chat_toolbar_narrow_button_width: 15px；search_plus_button_size: 28px
- toolbar_popover_width: 130px
- (to add) toolbar_item_padding: 10px；toolbar_item_radius: 6px
- (to add) bubble_padding_x: 10px；bubble_padding_y: 6px；bubble_radius: 6px
- (to add) session_row_height: 56px；session_row_padding_x: 12px
- (to add) list_row_gap: 6px；message_group_gap: 10px
- (to add) duration_fast: 100ms；duration_base: 150ms

## Coding Guidelines
- UI must not import infra; 仅依赖 ui::{theme,constants} + gpui/gpui-component；通过 app/state 提供数据。
- App 统一从 infra 拉取数据并注入到组件；组件不直接持有仓库或服务引用。
- Prefer small, pure helper functions/components for repeated UI fragments.

## Rollback Strategy
- infra::memory_repos 仍委托 infra::sample_data；切换数据源成本低。
- 主题直接使用 ui::theme；若需回退仅需调整 import。
- 本阶段均为无行为变更的重构。
