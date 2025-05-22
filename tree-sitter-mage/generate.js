const fs = require('fs');
const path = require('path');
const child_process = require('child_process');

// Get this script's directory
const currentDir = __dirname;
console.log('Current directory:', currentDir);

try {
  // Run tree-sitter generate using explicit path
  const result = child_process.execSync(
    'npx tree-sitter generate', 
    { 
      cwd: currentDir,
      env: { ...process.env, NODE_PATH: currentDir },
      stdio: 'inherit'
    }
  );
  
  console.log('Generation complete!');
} catch (error) {
  console.error('Error generating parser:', error.message);
  process.exit(1);
} 