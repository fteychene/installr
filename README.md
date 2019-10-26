# InstallR
Custom tool for autoinstall laptop

## Goal
InstallR is aimed to be a ingle executable to download and use directly on your machine that :
 * Clone a git repository
 * Read configuration from repository
 * Execute tasks and configuration

## Why InstallR 
> Why not use configuration software like ansible ?

Because ansible rely on python installation and InstallR goal is to be the most agnostic about your installation

## System dependencies

InstallR binary rely on the following system libraries :
 * `libssl`

#Todo

- [ ] Create CLI tool
- [ ] Download git repsository
- [ ] Read configuration from repository
- [ ] Execute tasks from configuration
- [ ] Add file link task
- [ ] Add script task
- [ ] Add user definition on tasks