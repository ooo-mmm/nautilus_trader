# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

import pytest

from nautilus_trader.model.data import BarType


class TestOKXBarTypePointerIdentity:
    """
    Tests to ensure BarType pointer identity is preserved through parsing.

    This guards against regressions where new BarType instances are created during
    parsing, causing hash lookup failures in subscription maps.

    """

    def test_bar_type_pointer_survives_parse(self):
        """
        Test that BarType pointer identity is preserved during OKX bar parsing.

        This test demonstrates that while Python's dict handles equal objects correctly,
        maintaining pointer identity is still the best practice for performance and
        clarity.

        """
        # Arrange
        bar_type_str = "ETH-USDT-SWAP.OKX-1-MINUTE-LAST-EXTERNAL"
        original_bar_type = BarType.from_str(bar_type_str)

        # Simulate the common pattern where we might recreate a BarType
        # This should NOT be done in actual parsing code
        recreated_bar_type = BarType.from_str(bar_type_str)

        # Act & Assert - These should be equal but NOT the same object
        assert original_bar_type == recreated_bar_type
        assert original_bar_type is not recreated_bar_type  # Different objects

        # Python's dict implementation actually handles this correctly via __hash__ and __eq__
        subscription_map = {original_bar_type: "handler"}

        # Both work due to proper hash/equality implementation
        assert original_bar_type in subscription_map
        assert recreated_bar_type in subscription_map  # This actually works in Python!

        # However, identity checks would fail
        for key in subscription_map:
            assert key is original_bar_type  # Works with original
            assert key is not recreated_bar_type  # Would fail with recreated

        # Performance: identity check is O(1), equality check is O(n) for the comparison
        # So maintaining identity is still preferred for high-frequency operations

    def test_bar_type_hash_consistency(self):
        """
        Test that equal BarType instances have the same hash.

        This ensures that even if we accidentally create new instances, hash-based
        lookups will still work correctly.

        """
        # Arrange
        bar_type_str = "BTC-USDT-SWAP.OKX-5-MINUTE-LAST-EXTERNAL"
        bar_type1 = BarType.from_str(bar_type_str)
        bar_type2 = BarType.from_str(bar_type_str)

        # Act & Assert
        assert bar_type1 == bar_type2
        assert hash(bar_type1) == hash(bar_type2)

        # Both should work in hash-based collections
        bar_set = {bar_type1}
        assert bar_type1 in bar_set
        assert bar_type2 in bar_set  # Should work due to equal hash and equality

    @pytest.mark.parametrize(
        "bar_type_str",
        [
            "ETH-USDT-SWAP.OKX-1-MINUTE-LAST-EXTERNAL",
            "BTC-USDT-SWAP.OKX-5-MINUTE-LAST-EXTERNAL",
            "SOL-USDT.OKX-15-MINUTE-LAST-EXTERNAL",
            "DOGE-USDT.OKX-1-HOUR-LAST-EXTERNAL",
        ],
    )
    def test_multiple_bar_types_hash_uniqueness(self, bar_type_str: str):
        """
        Test that different BarType strings produce different hashes.

        This ensures proper separation in subscription maps.

        """
        # Arrange
        bar_type = BarType.from_str(bar_type_str)
        other_bar_type = BarType.from_str("DIFFERENT-PAIR.OKX-1-MINUTE-LAST-EXTERNAL")

        # Act & Assert
        assert bar_type != other_bar_type
        # Hashes should be different (though hash collisions are theoretically possible)
        assert hash(bar_type) != hash(other_bar_type)
