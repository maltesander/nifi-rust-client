#!/bin/sh
set -e

# Source NiFi common utilities (provides prop_replace)
# shellcheck source=/dev/null
. /opt/nifi/scripts/common.sh

# Inject keystore/truststore settings into nifi.properties directly.
# This replicates what secure.sh does but without setting AUTH=tls,
# so Jetty does NOT enforce mutual TLS / client certificate authentication.
prop_replace 'nifi.security.keystore'           "${KEYSTORE_PATH}"
prop_replace 'nifi.security.keystoreType'       "${KEYSTORE_TYPE}"
prop_replace 'nifi.security.keystorePasswd'     "${KEYSTORE_PASSWORD}"
prop_replace 'nifi.security.keyPasswd'          "${KEY_PASSWORD:-$KEYSTORE_PASSWORD}"
prop_replace 'nifi.security.truststore'         "${TRUSTSTORE_PATH}"
prop_replace 'nifi.security.truststoreType'     "${TRUSTSTORE_TYPE}"
prop_replace 'nifi.security.truststorePasswd'   "${TRUSTSTORE_PASSWORD}"

# Configure single-user login credentials and switch to managed-authorizer.
#
# start.sh would normally call set-single-user-credentials and then set
# nifi.security.user.authorizer=single-user-authorizer, which blocks user/group
# management via the REST API. Instead, we:
#   1. Run set-single-user-credentials ourselves to hash the password into
#      login-identity-providers.xml and configure nifi.properties.
#   2. Override the authorizer to managed-authorizer (authorizers.xml is already
#      pre-configured with Initial Admin Identity=admin via a volume mount).
#   3. Unset the credentials env vars so start.sh skips its own invocation of
#      set-single-user-credentials and does not revert the authorizer.
if [ -n "${SINGLE_USER_CREDENTIALS_USERNAME}" ] && [ -n "${SINGLE_USER_CREDENTIALS_PASSWORD}" ]; then
    "${NIFI_HOME}/bin/nifi.sh" set-single-user-credentials \
        "${SINGLE_USER_CREDENTIALS_USERNAME}" \
        "${SINGLE_USER_CREDENTIALS_PASSWORD}"

    prop_replace 'nifi.security.user.authorizer' 'managed-authorizer'

    unset SINGLE_USER_CREDENTIALS_USERNAME
    unset SINGLE_USER_CREDENTIALS_PASSWORD
fi

# Delegate to the standard NiFi startup without AUTH, so no client cert
# is required at the TLS layer.
unset AUTH
exec /opt/nifi/scripts/start.sh
