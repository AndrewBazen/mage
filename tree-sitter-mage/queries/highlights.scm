;; Keywords
(variable_declaration "conjure" @keyword)
(output "incant" @keyword)
(error "curse" @keyword)
(command "evoke" @keyword)
(function_declaration "enchant" @keyword)
(function_call "cast" @keyword)
(if_statement "if" @keyword)
(if_statement "else" @keyword)
(loop_statement "loop" @keyword)

;; Operators
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
"<=" @operator
">" @operator
">=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator

;; Punctuation
"(" @punctuation.delimiter
")" @punctuation.delimiter
"{" @punctuation.delimiter
"}" @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter

;; Variables and Functions
(variable_declaration name: (identifier) @variable.declaration)
(parameter_list (identifier) @variable.parameter)
(function_declaration name: (identifier) @function)
(function_call name: (identifier) @function.call)

;; Literals
(string) @string
(number) @number
(boolean) @constant.builtin

;; Comments
(comment) @comment
(multiline_comment) @comment.block 