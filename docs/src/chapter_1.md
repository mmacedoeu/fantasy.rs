# Chapter 1

## Project structure

Breath of Fantasy is modularized from the start with [separation of concerns](https://en.wikipedia.org/wiki/Separation_of_concerns) in mind

```text
.
├── crates # root folder of modules
│   ├── app-dir # folder and app data management module
│   │   └── src
│   ├── bpm # business rules management module
│   │   └── src
│   ├── client # setup, glue code and startup module
│   │   └── src
│   ├── core # common struct and entities module
│   │   └── src
│   ├── engine # states definition and handling module
│   │   ├── src
│   │   └── tests
│   ├── engine-io # stdin stdout io connector module
│   │   └── src
│   └── fconfig # fantasy configuration module
│       └── src
├── docker # docker support
│   └── alpine_edge # alpine based Dockerfile
├── docs # Documentation
│   ├── book # target placeholder of book compilation
│   └── src # source archives of book
├── snap # snapd support
├── src
└── tests
```

* src folders are reserved for source code
* tests folders are reserved for test code

