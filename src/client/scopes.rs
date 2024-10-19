use std::ops::{BitOr, BitOrAssign};

/// Scopes bitflags for an [`Osu`] client.
///
/// To specify multiple scopes, create a union using the `|` operator.
///
/// See <https://osu.ppy.sh/docs/index.html#scopes>.
///
/// [`Osu`]: crate::Osu
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Scopes(u16);

macro_rules! define_scopes {
    ( $(
        #[doc = $doc:literal]
        $scope:ident: $shift:literal, $str:literal;
    )* ) => {
        define_scopes! {@ $(
            #[doc = $doc]
            $scope: 1 << $shift, $str;
        )* }
    };
    (@ $(
        #[doc = $doc:literal]
        $scope:ident: $bit:expr, $str:literal;
    )* ) => {
        $(
            #[allow(non_upper_case_globals)]
            impl Scopes {
                #[doc = $doc]
                pub const $scope: Self = Self($bit);
            }
        )*

        impl Scopes {
            const fn contains(self, bit: u16) -> bool {
                (self.0 & bit) > 0
            }

            pub(crate) fn format(self, s: &mut String, separator: char) {
                let mut first_scope = true;

                $(
                    if self.contains($bit) {
                        if !first_scope {
                            s.push(separator);
                        }

                        s.push_str($str);

                        #[allow(unused_assignments)]
                        {
                            first_scope = false;
                        }
                    }
                )*
            }
        }
    };
}

define_scopes! {
    /// Allows reading chat messages on a user's behalf.
    ChatRead: 0, "chat.read";
    /// Allows sending chat messages on a user's behalf.
    ChatWrite: 1, "chat.write";
    /// Allows joining and leaving chat channels on a user's behalf.
    ChatWriteManage: 2, "chat.write_manage";
    /// Allows acting as the owner of a client.
    Delegate: 3, "delegate";
    /// Allows creating and editing forum posts on a user's behalf.
    ForumWrite: 4, "forum.write";
    /// Allows reading of the user's friend list.
    FriendsRead: 5, "friends.read";
    /// Allows reading of the public profile of the user.
    Identify: 6, "identify";
    /// Allows reading of publicly available data on behalf of the user.
    Public: 7, "public";
}

impl Default for Scopes {
    fn default() -> Self {
        Self::Public
    }
}

impl BitOr for Scopes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
    }
}

impl BitOrAssign for Scopes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0);
    }
}
