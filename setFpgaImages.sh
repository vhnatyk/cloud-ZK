# clear both fpgas of old images
sudo fpga-clear-local-image -S 0
sudo fpga-clear-local-image -S 1

<<'###BLOCK-COMMENT'
New G1 250MHz build where padding is not needed. Input order is still [y, x] where x and y is 256b
{
    "FpgaImages": [
        {
            "FpgaImageId": "afi-05ff6a417ce1f8f76",
            "FpgaImageGlobalId": "agfi-01e758858844860d9",
            "Name": "rapidsnark-g1-f250",
            "State": {
                "Code": "pending"
            },
            "CreateTime": "2022-11-01T02:03:14.000Z",
            "UpdateTime": "2022-11-01T02:03:14.000Z",
            "OwnerId": "983370650745",
            "Tags": [],
            "Public": false,
            "DataRetentionSupport": false
        }
    ]
}
###BLOCK-COMMENT

# fpga @ slot 0 should be used for G1
sudo fpga-load-local-image -S 0 -I agfi-01e758858844860d9 -H

<<'###BLOCK-COMMENT'
New G2 125MHz build with bug fix.
{
    "FpgaImages": [
        {
            "FpgaImageId": "afi-065871d621fe1c7cd",
            "FpgaImageGlobalId": "agfi-0b9615a9adbd51774",
            "Name": "rapidsnark-g2-f125",
            "State": {
                "Code": "pending"
            },
            "CreateTime": "2022-11-01T02:10:25.000Z",
            "UpdateTime": "2022-11-01T02:10:25.000Z",
            "OwnerId": "983370650745",
            "Tags": [],
            "Public": false,
            "DataRetentionSupport": false
        }
    ]
}
###BLOCK-COMMENT
# fpga @ slot 1 should be used for G2
sudo fpga-load-local-image -S 1 -I agfi-0b9615a9adbd51774 -H