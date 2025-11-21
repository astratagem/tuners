# SPDX-FileCopyrightText: (C) 2022-2025 Chris Montgomery <chmont@protonmail.com>
# SPDX-FileCopyrightText: (C) 2022-2025 Temple University <kleinweb@temple.edu>
# SPDX-License-Identifier: GPL-3.0-or-later

prj-root := env("PRJ_ROOT")
owner-name := "chris montgomery"
owner-email := "chmont@protonmail.com"
owner-attribution := owner-name + " <" + owner-email + ">"

agpl3 := 'AGPL-3.0-or-later'
docs-license := "CC-BY-SA-4.0"
and := ' AND '

alias public-domain := cc0

lint:
    reuse lint

[doc("Annotate a file")]
annotate license +ARGS:
    reuse annotate \
    --copyright-prefix spdx-c \
    --license '{{license}}' \
    --copyright '{{owner-attribution}}' \
    --merge-copyrights \
    {{ARGS}}

[doc: "Annotate all unlicensed files"]
[no-cd]
annotate-missing task:
    reuse lint --json \
    | jq -r '.non_compliant.missing_licensing_info.[]' \
    | xargs -n1 -I{} just reuse::{{task}} {{prj-root}}/{}

[doc: "Annotate all plaintext note files with the documentation license"]
annotate-docs:
    fd --glob '**/*.{md,mdx,markdown,org}' -X \
    just commons {}

[doc: "License the specified files as GPL-3.0-or-later"]
gpl +FILES: (annotate agpl3 FILES)

[doc: "Add a co-licensor to the specified files"]
colicense license colicensor +FILES: (annotate (license) ('-c ' + quote(colicensor) + ' ' + FILES))

[doc: "License the specified files under an additional license"]
dual-license other-license +FILES: (annotate (agpl3 + and + other-license) FILES)

[doc: "License the specified files as documentation"]
docs +FILES: (annotate docs-license FILES)

[doc: "Release the specified files into the public domain"]
cc0 +FILES: (annotate 'CC0-1.0' FILES)
