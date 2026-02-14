;; Minimal completion query file for blink.cmp
;; This avoids syntax errors while providing basic completion support

;; Identifiers for completion
(identifier) @completion

;; Function names for completion
(function_declaration name: (identifier) @function)
(function_call name: (identifier) @function)

;; Keywords
"conjure" @keyword
"incant" @keyword
"curse" @keyword
"evoke" @keyword
"enchant" @keyword
"cast" @keyword
"if" @keyword
"else" @keyword
"loop" @keyword 