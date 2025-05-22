try {
  module.exports = require("../../build/Release/tree_sitter_mage_binding");
} catch (error1) {
  if (error1.code !== 'MODULE_NOT_FOUND') {
    throw error1;
  }
  try {
    module.exports = require("../../build/Debug/tree_sitter_mage_binding");
  } catch (error2) {
    if (error2.code === 'MODULE_NOT_FOUND') {
      throw error1;
    } else {
      throw error2;
    }
  }
} 