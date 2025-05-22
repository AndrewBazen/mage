module.exports = grammar({
  name: 'mage2',

  rules: {
    source_file: $ => repeat($._definition),

    _definition: $ => choice(
      $.variable_declaration,
      $.function_declaration,
      $.command,
      $.output,
      $.error,
      $.if_statement,
      $.loop_statement,
      $.function_call,
      $.comment,
      $.multiline_comment
    ),

    variable_declaration: $ => seq(
      'conjure',
      field('name', $.identifier),
      '=',
      field('value', $._expression)
    ),

    function_declaration: $ => seq(
      'enchant',
      field('name', $.identifier),
      '(',
      optional($.parameter_list),
      ')',
      field('body', $.block)
    ),

    parameter_list: $ => seq(
      $.identifier,
      repeat(seq(',', $.identifier))
    ),

    output: $ => seq(
      'incant',
      field('message', $.string)
    ),

    error: $ => seq(
      'curse',
      field('message', $.string)
    ),

    command: $ => seq(
      'evoke',
      field('command', choice($.string, $.identifier))
    ),

    function_call: $ => seq(
      'cast',
      field('name', $.identifier),
      '(',
      optional($.argument_list),
      ')'
    ),

    argument_list: $ => seq(
      $._expression,
      repeat(seq(',', $._expression))
    ),

    if_statement: $ => seq(
      'if',
      field('condition', $._expression),
      field('consequence', $.block),
      optional(seq(
        'else',
        field('alternative', choice($.block, $.if_statement))
      ))
    ),

    loop_statement: $ => seq(
      'loop',
      field('body', $.block)
    ),

    block: $ => seq(
      '{',
      repeat($._definition),
      '}'
    ),

    _expression: $ => choice(
      $.string,
      $.number,
      $.boolean,
      $.identifier,
      $.function_call,
      $.parenthesized_expression,
      $.binary_expression
    ),

    parenthesized_expression: $ => seq(
      '(',
      $._expression,
      ')'
    ),

    binary_expression: $ => {
      const table = [
        ['==', 'equal'],
        ['!=', 'not_equal'],
        ['<', 'less_than'],
        ['<=', 'less_than_equal'],
        ['>', 'greater_than'],
        ['>=', 'greater_than_equal'],
        ['+', 'add'],
        ['-', 'subtract'],
        ['*', 'multiply'],
        ['/', 'divide'],
      ];

      return choice(...table.map(([operator, name]) => {
        return prec.left(1, seq(
          field('left', $._expression),
          field('operator', operator),
          field('right', $._expression)
        ));
      }));
    },

    comment: $ => seq(
      '#',
      /[^\n]*/
    ),

    multiline_comment: $ => seq(
      '##',
      repeat(choice(
        seq('#', /[^\n]*/),
        '\n'
      )),
      '##'
    ),

    string: $ => choice(
      seq(
        '"',
        repeat(choice(
          $._string_content,
          $.string_interpolation
        )),
        '"'
      ),
      seq(
        "'",
        repeat($._string_content),
        "'"
      )
    ),

    _string_content: $ => choice(
      token.immediate(prec(1, /[^"\\$]+/)),
      $.escape_sequence
    ),

    string_interpolation: $ => choice(
      seq('$', $.identifier),
      seq('${', $.identifier, '}')
    ),

    escape_sequence: $ => token.immediate(seq(
      '\\',
      choice(
        /[\\'"$]/,
        /n/,
        /t/,
        /r/,
        /0/,
        /\{/,
        /\}/
      )
    )),

    number: $ => /\d+(\.\d+)?/,

    boolean: $ => choice('true', 'false'),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
  }
}); 