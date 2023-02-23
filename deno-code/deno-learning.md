# Deno command
- deno run : run a file,such as : deno run main.ts
- deno cache : Oprtating the cache of the project 

## Reloading Modules
Reload a remote files,use `deno cache --reload`.When you want reload files for a particular modules, use `deno cache --reload=https://deno.land/std@0.171.0`, if you have two modules need reload, use `deno cache --reload=https://deno.land/std@0.171.0 aaa.ts,https://deno.land/std@0.171.0 bbb.ts `.

## Integrity Checking and Lock Files
When we use remote files that have been changed, but we need the old version. In this situation, we need to lock our module version in deno.lock file. So we can use command `deno --lock=deno.lock` to lock the version of modules.

When we need upgrade or downgrade a version, we need use command `deno --lock=deno.lock --lock-write` to update the file content.

## Standard Library - Troubleshooting
When we need to use the fs module in the Standard Library, we need to use command `deno run --allow-read --allow-write --unstable`

## Permissions
If you want run the deno program, you need command like `--allow-read` to allow your computer to use. For example, if you want to request a piece of data through fetch, you need allow the `net` permission, you should use the command `deno run --allow-net xxx.ts` to execute it;

The permissions list:
- --allow-env
- --allow-sys
- --allow-hrtime
- --allow-net
- --allow-ffi
- --allow-read
- --allow-write
- --allow-run
- -A, --allow-all








