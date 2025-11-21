# SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

[confirm: "Prepare and tag release?"]
[doc: "Prepare and tag a release based on semantic commits"]
[no-cd]
release *ARGS='--auto':
  cog bump {{ARGS}}
