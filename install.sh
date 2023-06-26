# cp binary
cp ./target/debug/prompt-custormize /usr/bin/

# add zshrc
echo "PROMPT=\`echo -e \\\`/usr/bin/prompt-custormize\\\`\`" >> ~/.zshrc
