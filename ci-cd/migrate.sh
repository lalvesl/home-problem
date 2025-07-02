# if ! cargo &>/dev/null; then
#     echo "Cargo could not be found, install by 'https://www.rust-lang.org/' this documentation"
#     exit 1
# fi

if ! sea-orm-cli --version &>/dev/null; then
  cargo install sea-orm-cli --profile dev
fi

if [ -f ".env" ]; then
  export $(cat .env | grep -v '^#' | grep "DB_\(CDP\)\|\(SALES_HUB\)")
fi

if [ -z $DB_CDP_PG_DB ]; then
  echo "Enviroment variables not load"
  exit 1
fi

if ! rustfmt --version &>/dev/null; then
  rustsetup component add rustfmt
fi

for schema in $(echo "public cdp quotes"); do
  locality="db/$schema"

  sea-orm-cli \
    migrate \
    $1 $2 $3 $4 $5 $6 $7 $8 $9 \
    -d "$locality" \
    -s "$schema" \
    -u \
    "postgresql://$DB_CDP_PG_USER:$(echo $DB_CDP_PG_PASS |
      base64 -d |
      jq -sRr @uri)@$DB_CDP_PG_HOST:$DB_CDP_PG_PORT/$DB_CDP_PG_DB"

done
