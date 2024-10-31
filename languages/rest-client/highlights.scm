;; Based on the nvim-treesitter highlighting, which is under the Apache license.
;; See https://github.com/nvim-treesitter/nvim-treesitter/blob/f8ab59861eed4a1c168505e3433462ed800f2bae/queries/kotlin/highlights.scm
;;
;; The only difference in this file is that queries using #lua-match?
;; have been removed.

;;; Identifiers

(simple_identifier) @variable

; `GET` keyword
((simple_identifier) @variable.builtin
(#eq? @variable.builtin "GET"))
