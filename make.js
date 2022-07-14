const fs = require('fs')
fs.writeFileSync('future-generations.old.json', JSON.stringify(require('./future-generations.json'), undefined, 2))
fs.writeFileSync('future-generations.new.json', JSON.stringify(Object.fromEntries(require('./new-future-generations.json')), undefined, 2))
