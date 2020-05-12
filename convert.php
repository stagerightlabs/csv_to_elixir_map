<?php

use League\Csv\Reader;
use League\Csv\Statement;

require __DIR__ . '/vendor/autoload.php';

// ensure we have been provided a path argument
if (empty($argv[1])) {
    echo "please specify a file path.\n";
    return 1;
}

// Attempt to read the csv file
try {
    $csv = Reader::createFromPath($argv[1], 'r');
    $csv->setHeaderOffset(0);
} catch (\Throwable $th) {
    echo $th->getMessage();
    return 1;
}

// Extract the records from the $csv object
$records = Statement::create()->process($csv);

// Generate our text content
$output = collect($records->getRecords())
    ->reduce(function($carry, $r) {
        $map = create_map($r);
        return $carry . "  %{{$map}},\n";
    }, '');

// Remove the last comma and newline
$output = rtrim($output, ",\n");

// Create our output file name
$outputFilename = empty($argv[2])
    ? generateRandomString('.txt')
    : $argv[2];

// Write the output to a file
$fileHandle = fopen($outputFilename, 'w');
fwrite($fileHandle,"[\n$output\n]");
fclose($fileHandle);

echo "created {$outputFilename}\n";
return 0;

/**
 * Convert a hash map into a string of "key: value," entries
 *
 * @param array $arr
 * @return string
 */
function create_map($arr) {
    return collect($arr)
        ->map(function($v, $k) {
            return "{$k}: \"{$v}\"";
        })
        ->implode(', ');
}


/**
 * Generate a random string. https: //stackoverflow.com/a/13212994
 *
 * @param integer $length
 * @return string
 */
function generateRandomString($suffix = '', $length = 10)
{
    return substr(str_shuffle(str_repeat($x = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', ceil($length / strlen($x)))), 1, $length) . $suffix;
}
