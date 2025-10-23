use gpui::prelude::FluentBuilder;
use gpui::{div, IntoElement, ParentElement, Styled};
use gpui_component::{avatar::Avatar, badge::Badge, Sizable};

use crate::models::Contact;
use crate::ui::widgets::group_avatar::GroupAvatar;

pub fn badge_avatar(
    contact: &Contact,
    weixin_colors: &crate::ui::theme::WeixinThemeColors,
) -> impl IntoElement {
    div()
        .flex_shrink_0()
        .when(contact.unread_count > 0, |this| {
            this.child(
                Badge::new()
                    .count(contact.unread_count as usize)
                    .max(99)
                    .color(weixin_colors.unread_badge)
                    .when(contact.is_group, |badge| {
                        badge.child(GroupAvatar::new(contact.avatar_members.clone()))
                    })
                    .when(!contact.is_group, |badge| {
                        badge.child(
                            div().overflow_hidden().child(
                                Avatar::new()
                                    .rounded(crate::ui::constants::avatar_small_radius())
                                    .with_size(crate::ui::constants::avatar_large())
                                    .src(crate::ui::avatar::avatar_for_key(&contact.id)),
                            ),
                        )
                    }),
            )
        })
        .when(contact.unread_count == 0, |this| {
            this.when(contact.is_group, |div_| {
                div_.child(GroupAvatar::new(contact.avatar_members.clone()))
            })
            .when(!contact.is_group, |div_| {
                div_.child(
                    div().overflow_hidden().child(
                        Avatar::new()
                            .rounded(crate::ui::constants::avatar_small_radius())
                            .with_size(crate::ui::constants::avatar_large())
                            .src(crate::ui::avatar::avatar_for_key(&contact.id)),
                    ),
                )
            })
        })
}
