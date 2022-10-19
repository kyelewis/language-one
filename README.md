# onelang

Because everyone should have their own language.

## Summary
`onelang` is a language focused on built-in communication channels and conversion between formats.

It aims to be highly configurable to only build the communications channels and formats necessary.

```sh
echo "say `Hello, World!`" > helloworld.one
onelang --file helloworld.one
Hello, World!
```

## Example Program
```
ask `What format would you like to return your IP in? (json, text)` format;
http_get `https://api.ipify.org/?format=$format` ip_address;
say `Your IP address is $ip_address`;
exit 0;
```
