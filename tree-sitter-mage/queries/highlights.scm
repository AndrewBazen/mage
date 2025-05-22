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

;; Operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator

;; Punctuation
"(" @punctuation.delimiter
")" @punctuation.delimiter
"{" @punctuation.delimiter
"}" @punctuation.delimiter
";" @punctuation.delimiter
"," @punctuation.delimiter

;; Literals
(string) @string
(number) @number

;; Variables and Functions
(variable_declaration name: (identifier) @variable.declaration)
(parameter_list (identifier) @variable.parameter)
(function_declaration name: (identifier) @function)
(function_call name: (identifier) @function.call)

;; Comments
(comment) @comment
(multiline_comment) @comment.block 