# Nebula contracts automated tests tool

This is a tool for automated tests of Nebula contracts written in node.js. Contract tests are located in the `contracts` directory - each test file is a different test scenario. One contract can have several test files (scenarios). All of them should extend the `BaseTest` class. 

In the test scenarios the following steps are performed:
- create accounts
- deploy contract
- invoke contract methods
- check results
- calculate contract fees (both deployment and method invoke)
If one of the steps fail, the whole scenario is considered failed. 

## Running tests
To run the tests you need node.js and npm. First you need to install all required packages:
```
npm install
```

Then you can start tests:
```
npm run test
```

You can also run specific test:
```
node --loader tsx --test contracts/voting.test.ts
```

For more commands see: https://nodejs.org/api/test.html
